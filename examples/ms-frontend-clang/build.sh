#!/bin/bash

set -e

CMAKE_SOURCE_DIR=$(cd "$(dirname $0)"; pwd)
if [ ! -d "${CMAKE_SOURCE_DIR}/build" ]; then
    mkdir -p ${CMAKE_SOURCE_DIR}/build
fi

cd ${CMAKE_SOURCE_DIR}/build
cmake -D CMAKE_CXX_FLAGS="-fno-exceptions -fno-rtti" ..
cmake --build . -j4
