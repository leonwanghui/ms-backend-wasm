#!/bin/bash

set -e

CAMKE_SOURCE_DIR=$(cd dirname $0; pwd)
XNNPACK_SOURCE_DIR=${CAMKE_SOURCE_DIR}/third_party/xnnpack

if [ ! -d "${XNNPACK_SOURCE_DIR}/build" ]; then
    mkdir -p ${XNNPACK_SOURCE_DIR}/build
fi

cd ${XNNPACK_SOURCE_DIR}/build
cmake -DCMAKE_INSTALL_PREFIX:PATH=./install .. -j4
cmake --build . --target install -j4

cd ${CAMKE_SOURCE_DIR}/build
cmake --build . -j4
