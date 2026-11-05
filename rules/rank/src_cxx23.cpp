#include <algorithm>
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::rank<T1>;

template <typename T1> std::size_t f1(const std::rank<T1> &a0) {
  return static_cast<std::size_t>(a0);
}

template <typename T1> std::size_t f2(const std::rank<T1> &a0) {
  return a0.operator()();
}

template <typename T1> std::size_t f3() { return std::rank<T1>::value; }
