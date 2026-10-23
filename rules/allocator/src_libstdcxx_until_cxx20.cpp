#include <string>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::allocator<T1> f18(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}

template <typename T1>
bool f19(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return operator!=(a0, a1);
}
