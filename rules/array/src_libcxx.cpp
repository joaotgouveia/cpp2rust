#include <array>
#include <cstddef>
#include <iterator>
#include <utility>

template <typename T1, std::size_t T2,
          std::__enable_if_t<T2 == 0 || std::__is_swappable_v<T1>, int> T3 = 0>
void f43(std::array<T1, T2> &a0, std::array<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
