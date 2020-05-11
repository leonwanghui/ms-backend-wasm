#!/bin/bash

set -e

WASI_SDK_PATH=${WASI_SDK_PATH:-/usr/local/wasi-sdk-10.0}
if [ ! -d "${WASI_SDK_PATH}" ]; then
    echo "${WASI_SDK_PATH} doesn't exist!"
    exit 1
fi
export CC="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export CXX="${WASI_SDK_PATH}/bin/clang++ --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"

CMAKE_SOURCE_DIR=$(cd "$(dirname $0)"; pwd)
XNNPACK_SOURCE_DIR=${CMAKE_SOURCE_DIR}/third_party/xnnpack

if [ ! -d "${XNNPACK_SOURCE_DIR}/build" ]; then
    mkdir -p ${XNNPACK_SOURCE_DIR}/build
fi
cd ${XNNPACK_SOURCE_DIR}/build
cmake -DCMAKE_INSTALL_PREFIX:PATH=./install ..
cmake --build . --target install -j4

if [ ! -d "${CMAKE_SOURCE_DIR}/build" ]; then
    mkdir -p ${CMAKE_SOURCE_DIR}/build
fi
cd ${CMAKE_SOURCE_DIR}/build
cmake -D CMAKE_CXX_FLAGS="-fno-exceptions -fno-rtti" ..
cmake --build . -j4
