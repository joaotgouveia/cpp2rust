#include <array>
#include <cstddef>

template <typename T1, std::size_t T2>
std::array<T1, T2> f1(const T1 (&a0)[T2]) {
  return {a0};
}

template <typename T1> std::array<T1, 0> f2() { return {}; }
