#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1>
typename std::is_nothrow_copy_assignable<T1>::value_type
f1(const std::is_nothrow_copy_assignable<T1> &a0) {
  return static_cast<typename std::is_nothrow_copy_assignable<T1>::value_type>(
      a0);
}
