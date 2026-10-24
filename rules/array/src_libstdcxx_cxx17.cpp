#include <algorithm>
#include <array>
#include <utility>

template <typename T1, std::size_t T2>
std::__enable_if_t<std::__array_traits<T1, T2>::_Is_swappable::value>
f42(std::array<T1, T2> &a0, std::array<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
