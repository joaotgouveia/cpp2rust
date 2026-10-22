#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1, std::size_t T2>
typename std::extent<T1, T2>::value_type f4(const std::extent<T1, T2> &a0) {
  return a0.operator()();
}
