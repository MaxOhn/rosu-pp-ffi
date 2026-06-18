use std::mem;

macro_rules! handle {
    (
        $( #[ $meta:meta ] )*
        $name:ident -> $inner:ty
    ) => {
        $( #[ $meta ] )*
        pub struct $name($inner);

        impl From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }

        impl crate::handle::Handle for $name {
            type Inner = $inner;

            fn apply_owned<F>(mut self: Box<Self>, f: F) -> Box<Self>
            where
                F: FnOnce(Self::Inner) -> Self::Inner,
            {
                self.0 = f(self.0);

                self
            }
        }
    };
}

pub trait Handle: From<Self::Inner> {
    type Inner;

    fn apply_owned<F>(self: Box<Self>, f: F) -> Box<Self>
    where
        F: FnOnce(Self::Inner) -> Self::Inner;
}

pub trait HandleRef {
    type InnerRef;

    fn checked_by_ref<'a>(self) -> Option<&'a Self::InnerRef>;

    fn by_ref<'a>(self) -> &'a Self::InnerRef;
}

pub trait HandleMut {
    type InnerMut;

    fn by_mut<'a>(self) -> &'a mut Self::InnerMut;
}

pub trait HandleOwned: Sized {
    type InnerOwned;

    fn by_owned<F>(self, f: F)
    where
        F: FnOnce(Self::InnerOwned) -> Self::InnerOwned;

    fn into_owned(self) -> Box<Self::InnerOwned>;

    fn drop_handle(self);
}

impl<H: Handle> HandleRef for *const H {
    type InnerRef = H::Inner;

    fn checked_by_ref<'a>(self) -> Option<&'a Self::InnerRef> {
        unsafe { self.cast::<Self::InnerRef>().as_ref() }
    }

    fn by_ref<'a>(self) -> &'a Self::InnerRef {
        unsafe { &*self.cast::<Self::InnerRef>() }
    }
}

impl<H: Handle> HandleMut for *mut H {
    type InnerMut = H::Inner;

    fn by_mut<'a>(self) -> &'a mut Self::InnerMut {
        unsafe { &mut *self.cast::<Self::InnerMut>() }
    }
}

impl<H: Handle> HandleOwned for *mut H {
    type InnerOwned = H::Inner;

    fn by_owned<F>(self, f: F)
    where
        F: FnOnce(Self::InnerOwned) -> Self::InnerOwned,
    {
        let boxed = unsafe { Box::from_raw(self) };
        mem::forget(boxed.apply_owned(f));
    }

    fn into_owned(self) -> Box<Self::InnerOwned> {
        unsafe { Box::from_raw(self.cast::<Self::InnerOwned>()) }
    }

    fn drop_handle(self) {
        if !self.is_null() {
            drop(unsafe { Box::from_raw(self) });
        }
    }
}
