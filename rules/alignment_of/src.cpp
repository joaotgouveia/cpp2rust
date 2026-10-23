#include <algorithm>
#include <cstddef>
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::alignment_of<T1>;

template <typename T1> std::size_t f1(const std::alignment_of<T1> &a0) {
  return static_cast<std::size_t>(a0);
}
