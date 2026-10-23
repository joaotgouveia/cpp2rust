#include <algorithm>
#include <utility>

template <typename T1> T1 *f1(T1 &a0) {
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1>
T1 f2(typename std::pointer_traits<T1>::element_type &a0) {
  return std::pointer_traits<T1>::pointer_to(a0);
}
