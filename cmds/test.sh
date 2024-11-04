#!/bin/bash
# Needs to be in the env with the correct dependencies available (pytest, cargo etc)
success=true
pytest -s ./bindings/py/ && cargo test --features cli --features multithreading  || success=false
python -c "import pytest" || (echo "Failed to get pytest Python module." && exit 1)

if $success; then
    echo "All tests succeeded."
    exit 0
else
    echo "One or more tests failed."
    exit 1
fi