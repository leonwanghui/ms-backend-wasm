cmake_minimum_required(VERSION 3.14 FATAL_ERROR)

set(PROJECT_NAME ms-backend-wasm)
project(${PROJECT_NAME} LANGUAGES C CXX)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

find_library(XNNPACK_LIBRARY XNNPACK
    PATHS "${CMAKE_SOURCE_DIR}/third_party/xnnpack/build/install/lib"
    NO_DEFAULT_PATH
)
if (XNNPACK_NOTFOUND)
    message(FATAL_ERROR "XNNPACK Library Not Found!")
endif ()

file(GLOB SOURCE "${CMAKE_SOURCE_DIR}/src/*.cc")
include_directories("${CMAKE_SOURCE_DIR}/include")
include_directories("${CMAKE_SOURCE_DIR}/third_party/xnnpack/build/install/include")

add_executable(ms-backend-wasm.wasm ${SOURCE})
target_link_libraries(ms-backend-wasm.wasm ${XNNPACK_LIBRARY})
