#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, T1 T2>
typename std::integral_constant<T1, T2>::value_type
f3(const std::integral_constant<T1, T2> &a0) {
  return a0.operator()();
}
