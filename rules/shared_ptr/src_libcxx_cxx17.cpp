#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
__add_lvalue_reference(__remove_extent(T1))
f103(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
__add_lvalue_reference(__remove_extent(T1)) f104(const std::shared_ptr<T1> &a0,
                                                 std::ptrdiff_t a1) {
  return a0.operator[](a1);
}
