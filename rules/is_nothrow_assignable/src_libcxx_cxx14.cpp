#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::is_nothrow_assignable<T1, T2>::value_type
f2(const std::is_nothrow_assignable<T1, T2> &a0) {
  return a0.operator()();
}
