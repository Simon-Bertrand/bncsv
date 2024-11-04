#!/bin/bash
# This script is used to set up the build environment for the project.
python --version >/dev/null || (echo "Failed to find Python." && exit 1)
if [ ! -d ".venv" ]; then
    python -m venv .venv
fi

if [[ "$OSTYPE" == "msys" ]]; then
    source .venv/Scripts/activate
else
    source .venv/bin/activate
fi
python -m pip install -q --upgrade pip
pip install -q --force-reinstall ./bindings/py/
pip install pytest
python -c "import bncsv_py" || (echo "Failed to install bncsv_py Python module." && exit 1)
cargo --version >/dev/null || (echo "Failed to find cargo executable." && exit 1)
export BNCSV_BUILD_ENV=1