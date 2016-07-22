#!/bin/bash

declare -a targets

targets+=("x86_64-unknown-linux-gnu")
targets+=("i686-unknown-linux-gnu")
targets+=("i586-unknown-linux-gnu")
targets+=("mips-unknown-linux-gnu")
targets+=("mipsel-unknown-linux-gnu")
targets+=("powerpc-unknown-linux-gnu")
targets+=("powerpc64-unknown-linux-gnu")
targets+=("powerpc64le-unknown-linux-gnu")
targets+=("arm-unknown-linux-gnueabi")
targets+=("arm-unknown-linux-gnueabihf")
targets+=("armv7-unknown-linux-gnueabihf")
targets+=("aarch64-unknown-linux-gnu")
targets+=("x86_64-unknown-linux-musl")
targets+=("i686-unknown-linux-musl")
targets+=("mips-unknown-linux-musl")
targets+=("mipsel-unknown-linux-musl")

targets+=("i686-linux-android")
targets+=("arm-linux-androideabi")
targets+=("armv7-linux-androideabi")
targets+=("aarch64-linux-android")

targets+=("i686-unknown-freebsd")
targets+=("x86_64-unknown-freebsd")

targets+=("i686-unknown-dragonfly")
targets+=("x86_64-unknown-dragonfly")

targets+=("x86_64-unknown-bitrig")
targets+=("x86_64-unknown-openbsd")
targets+=("x86_64-unknown-netbsd")
targets+=("x86_64-rumprun-netbsd")

targets+=("x86_64-apple-darwin")
targets+=("i686-apple-darwin")

targets+=("i386-apple-ios")
targets+=("x86_64-apple-ios")
targets+=("aarch64-apple-ios")
targets+=("armv7-apple-ios")
targets+=("armv7s-apple-ios")

targets+=("x86_64-sun-solaris")

targets+=("x86_64-pc-windows-gnu")
targets+=("i686-pc-windows-gnu")

targets+=("x86_64-pc-windows-msvc")
targets+=("i686-pc-windows-msvc")
targets+=("i586-pc-windows-msvc")
targets+=("le32-unknown-nacl")
targets+=("asmjs-unknown-emscripten")

native_arch=$(grep 'CFG_BUILD ' config.mk | awk '{ print $3 }')
rustc="$(pwd)/${native_arch}/stage1/bin/rustc"
dest="src/librustc_back/target/json"
mkdir -p "${dest}"
for tgt in ${targets[@]}; do
    ${rustc} \
        --print target-spec \
        --target ${tgt} > "${dest}/${tgt}.json"
done
