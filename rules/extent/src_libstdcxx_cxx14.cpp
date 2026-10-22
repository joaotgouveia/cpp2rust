#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1, unsigned int T2>
typename std::extent<T1, T2>::value_type f2(const std::extent<T1, T2> &a0) {
  return a0.operator()();
}
