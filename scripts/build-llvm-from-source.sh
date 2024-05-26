#!/bin/sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
LLVM_REP_DIR="$SCRIPT_DIR/llvm/"
echo "Cloning llvm..."
git clone https://github.com/llvm/llvm-project $LLVM_REP_DIR
cd $LLVM_REP_DIR
git checkout tags/llvmorg-19-init
echo "Configuring..."
cmake -G Ninja -S llvm -B build \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX=../cache/ \
    -DLLVM_ENABLE_PROJECTS=clang \
    -DLLVM_ENABLE_RUNTIMES="libcxx;libcxxabi;libunwind"
echo "Building ..."
ninja -C build runtimes

