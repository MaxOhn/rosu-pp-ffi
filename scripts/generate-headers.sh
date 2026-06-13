#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Generating C headers with cheadergen..."
cd "$PROJECT_DIR"

# Remove old header if present
rm -f include/rosu_pp.h include/rosu_pp_ffi.h

# Generate headers
cheadergen generate --output-dir include --bundle

# Rename to expected filename
if [ -f "include/rosu_pp_ffi.h" ]; then
    mv "include/rosu_pp_ffi.h" "include/rosu_pp.h"
fi

echo "Headers generated in include/"
