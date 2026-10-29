#include <algorithm>
#include <chrono>
#include <utility>

void *f1(std::size_t a0, std::size_t a1, void *&a2, std::size_t &a3) {
  return std::align(a0, a1, a2, a3);
}

template <std::size_t T1, typename T2> T2 *f2(T2 *a0) {
  return std::assume_aligned<T1, T2>(a0);
}
