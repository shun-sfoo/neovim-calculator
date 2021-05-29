#!/usr/bin/env sh
set -o errexit

name=neovim-calculator

cargo_build() {
    if command -v cargo > /dev/null; then
        echo "Trying to build locally using Cargo.."
        cargo build --release
    else
        echo "Could not build binary. Your installation might be corrupt."
        return 1
    fi
}

arch=$(uname)
case "${arch}" in
    *) echo "No pre-built binary available for ${arch}."; cargo_build ;;
esac
