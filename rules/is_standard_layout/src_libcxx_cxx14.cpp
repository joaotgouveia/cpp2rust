#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::is_standard_layout<T1>::value_type
f2(const std::is_standard_layout<T1> &a0) {
  return a0.operator()();
}
