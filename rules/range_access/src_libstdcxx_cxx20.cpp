#include <algorithm>
#include <array>
#include <initializer_list>
#include <utility>

template <typename T1> auto f28(const T1 &a0) { return std::ssize<T1>(a0); }

template <typename T1, std::ptrdiff_t T2>
std::ptrdiff_t f29(const T1 (&a0)[T2]) {
  return std::ssize<T1, T2>(a0);
}
