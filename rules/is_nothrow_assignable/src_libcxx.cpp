#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::is_nothrow_assignable<T1, T2>::value_type
f1(const std::is_nothrow_assignable<T1, T2> &a0) {
  return static_cast<typename std::is_nothrow_assignable<T1, T2>::value_type>(
      a0);
}
