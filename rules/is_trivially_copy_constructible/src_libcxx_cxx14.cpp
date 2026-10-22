#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1>
typename std::is_trivially_copy_constructible<T1>::value_type
f2(const std::is_trivially_copy_constructible<T1> &a0) {
  return a0.operator()();
}
