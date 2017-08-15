#!/bin/bash -e

REQUIRED_TOOLS="virtualenv2 cargo"

PY_LIBRARIES="plyplus six tempita pyelftools orderedset jinja2 future"
cd "$( dirname "${BASH_SOURCE[0]}" )"
virtualenv2 . || ( echo "Please install virtualenv2" && exit 99 )
source ./bin/activate
for lib in $PY_LIBRARIES
do
    pip install $lib
done
(
    cd tools/xargo && cargo build --release
)
make
