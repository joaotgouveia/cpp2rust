#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1, typename T2>
typename std::uses_allocator<T1, T2>::value_type
f7(const std::uses_allocator<T1, T2> &a0) {
  return a0.operator()();
}
