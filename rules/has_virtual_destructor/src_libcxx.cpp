#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1>
typename std::has_virtual_destructor<T1>::value_type
f1(const std::has_virtual_destructor<T1> &a0) {
  return static_cast<typename std::has_virtual_destructor<T1>::value_type>(a0);
}
