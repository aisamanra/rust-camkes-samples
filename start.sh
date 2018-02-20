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
    rm -rf rust &&
        mkdir rust &&
        cd rust &&
        curl -O https://infinitenegativeutility.com/rust-x86_64.tar.gz &&
        tar -xf rust-x86_64.tar.gz
)

(
    rm -rf sysroot &&
        mkdir sysroot &&
        cd sysroot &&
        curl -O https://infinitenegativeutility.com/i686-sel4-robigalia.tar.gz &&
        tar -xf i686-sel4-robigalia.tar.gz
)

mkdir -p .cargo
cat >.cargo/config <<EOF
[build]
target = "i686-sel4-robigalia"
rustflags = ["--sysroot", "sysroot"]
rustc = "rust/bin/rustc"
EOF

make
