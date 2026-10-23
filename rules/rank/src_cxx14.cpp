#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::rank<T1>::value_type f2(const std::rank<T1> &a0) {
  return a0.operator()();
}
