#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1>
typename std::is_null_pointer<T1>::value_type
f1(const std::is_null_pointer<T1> &a0) {
  return a0.operator()();
}
