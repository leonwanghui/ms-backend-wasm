cmake_minimum_required(VERSION 3.14)

set(PROJECT_NAME ms-backend-wasm)
project(${PROJECT_NAME})

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")



file(GLOB SOURCE_1 "${CMAKE_SOURCE_DIR}/src/*.cc")
include_directories("${CMAKE_SOURCE_DIR}/include")
include_directories("${CMAKE_CURRENT_SOURCE_DIR}/third_party/xnnpack/include")

add_executable(ms-backend-wasm ${SOURCE_1})
