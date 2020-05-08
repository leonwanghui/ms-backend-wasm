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
extern "C" {

void Add(const size_t a_id, const size_t* a_shape_ptr, const size_t a_shape_len,
         const size_t b_id, const size_t* b_shape_ptr, const size_t b_shape_len,
         const DType dtype, const size_t out_id) {
  switch (dtype) {
    // case DType::float32:
    //   binary_xnn_f32(a_id, a_shape_ptr, a_shape_len, b_id, b_shape_ptr,
    //                  b_shape_len, out_id, xnn_create_add_nd_f32,
    //                  xnn_setup_add_nd_f32);
    //   break;
    // case DType::int32:
    //   binary_i32(a_id, b_id, out_id, add<int32_t>);
    //   break;
    // case DType::boolean:
    //   binary_bool(a_id, b_id, out_id, add<bool>);
    //   break;
    default:
      break;
  }
}
}  // extern "C"
}  // namespace wasm
}  // namespace mindspore

int main() {
    std::count<<"hello, world"<<std::endl;
    return 0;
}
