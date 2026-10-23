#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::pointer_traits<T1 *>::pointer
f1(typename std::pointer_traits<T1 *>::element_type &a0) {
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1>
typename std::pointer_traits<T1>::pointer
f2(typename std::pointer_traits<T1>::element_type &a0) {
  return std::pointer_traits<T1>::pointer_to(a0);
}
