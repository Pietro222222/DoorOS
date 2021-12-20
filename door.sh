#!/bin/bash
function door() {
    if [[ $# < 1 ]]; then
        echo "./door.sh <?>"
        echo "./door.sh build"
        echo "./door.sh setup"
        echo "./door.sh execute"
        return
    fi
    echo "[RUST/CHECK] Checking Rust Version..."
    if ! echo $(rustc --version) | grep "nightly"; then
        echo "[RUST/CHECK] DoorOS Requires Rust Nightly."
        return 
    fi
    if [[ $1 == 'setup' ]]; then 
        export CARGO_TARGET_DIR="$PWD/target"

        echo "[DOOR_OS/SETUP] Preparing... "
        rustup component add llvm-tools-preview
        FPATH="$PWD/targets/x86_64-unknown-none.json"
        echo "
[unstable]
build-std = [\"core\", \"compiler_builtins\", \"alloc\"]
[build]
target =  \"$FPATH\"
" > kernel/.cargo/config.toml
    echo "[DOOR_OS/SETUP] Installing bootimage"
    cargo install bootimage -vvv
    fi
    if [[ $1 == 'build' ]]; then
        export CARGO_TARGET_DIR="$PWD/target"
        
        pushd kernel    
            cargo build --verbose
            cargo bootimage
        popd
    fi
    if [[ $1 == 'execute' ]]; then
         qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-kernel.bin
    fi
}
door $@
