#include <ms-backend-wasm.h>

#include <xnnpack.h>
#include <iostream>

namespace {
template <class T>
inline T add(T a, T b) {
  return a + b;
}
}  // namespace

namespace mindspore {
namespace wasm {
// We use C-style API to interface with Javascript.
void Add() {
  xnn_initialize(nullptr /* allocator */);
  xnn_operator_t add_op = nullptr;

  xnn_run_operator(add_op, nullptr /* thread pool */);
}
}  // namespace wasm
}  // namespace mindspore

int main() {
    std::cout<<"hello, world"<<std::endl;
    mindspore::wasm::Add();
    return 0;
}
