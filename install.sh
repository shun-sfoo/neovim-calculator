#!/usr/bin/env sh
# Try install by
#   - download binary
#   - build with cargo

set -o nounset    # error when referencing undefined variable
set -o errexit    # exit when command fails

cargo_build() {
    if command -v cargo > /dev/null; then
        echo "Trying to build locally using Cargo.."
        cargo build --release
    else
        echo "Could not build binary. Your installation might be corrupt."
        return 1
    fi
}

arch=$(uname -sm)

case "${arch}" in
    "Darwin x86_64") echo "No pre-built binary available for ${arch}."; cargo_build ;;
    *) echo "No pre-built binary available for ${arch}."; cargo_build ;;
esac
