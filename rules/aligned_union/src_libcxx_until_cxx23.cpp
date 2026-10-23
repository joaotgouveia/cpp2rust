#include <cstddef>
#include <type_traits>
#include <utility>

template <std::size_t T1, typename T2, typename... T3>
using t3 = std::aligned_union<T1, T2, T3...>;

template <std::size_t T1, typename T2, typename... T3>
using t4 = typename std::aligned_union<T1, T2, T3...>::type;

template <std::size_t T1, typename T2, typename... T3> std::size_t f2() {
  return std::aligned_union<T1, T2, T3...>::alignment_value;
}
