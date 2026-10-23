#include <cstddef>
#include <type_traits>
#include <utility>

template <typename T1, std::size_t T2> using t2 = std::extent<T1, T2>;

template <typename T1, std::size_t T2>
std::size_t f3(const std::extent<T1, T2> &a0) {
  return static_cast<std::size_t>(a0);
}
