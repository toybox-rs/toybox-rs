#!/bin/bash

set -eu

# delete old rust wheels
rm -rf ctoybox/ctoybox
rm -rf ctoybox/*.abi3.so

# build rust package locally
maturin develop --release
# run unit tests

export PYTHONFAULTHANDLER=1
python -m unittest discover -s tests -v
