
#!/bin/bash
if [ "$BNCSV_BUILD_ENV" != "1" ]; then
    echo "Error: This script must be run in the BNCSV build environment"
    exit 1
fi

# Define target directories
TARGET_DIR="build"
RUST_SUBDIR="$TARGET_DIR/bncsv-core"
PYTHON_SUBDIR="$TARGET_DIR/bncsv_py"

# Create target directories if they don't exist
mkdir -p $TARGET_DIR
rm -rf $TARGET_DIR/*
mkdir -p $RUST_SUBDIR
mkdir -p $PYTHON_SUBDIR

build_rust() {
    echo "Building Rust project..."
    cargo clean
    cargo build --release --workspace --features cli,multithreading 
    if [ $? -eq 0 ]; then
        echo "Moving Rust build to $RUST_SUBDIR"
        mv target/release/bncsv* target/release/rs_api* $RUST_SUBDIR
    else
        echo "Rust build failed"
        exit 1
    fi
}

build_python() {
    echo "Building Python project..."
    python -m pip wheel ./bindings/py --wheel-dir $PYTHON_SUBDIR
    if [ $? -eq 0 ]; then
        echo "Python build successful"
    else
        echo "Python build failed"
        exit 1
    fi
}

# Check for user input
if [ "$1" == "rust" ]; then
    build_rust
elif [ "$1" == "python" ]; then
    build_python
else
    build_rust
    build_python
fi