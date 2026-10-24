#include <algorithm>
#include <string>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::allocator<T1> f23(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
bool f24(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
T1 const *f25(const std::allocator<T1> &a0, T1 const &a1) {
  return a0.address(a1);
}

template <typename T1, typename T2> void f26(std::allocator<T1> &a0, T2 *a1) {
  return a0.template destroy<T2>(a1);
}
