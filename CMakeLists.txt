cmake_minimum_required(VERSION 3.14 FATAL_ERROR)

set(PROJECT_NAME ms-backend-wasm)
project(${PROJECT_NAME} LANGUAGES C CXX)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

file(GLOB SOURCE "${CMAKE_SOURCE_DIR}/src/*.cc")
include_directories("${CMAKE_SOURCE_DIR}/include")

add_executable(ms-backend-wasm.wasm ${SOURCE})
