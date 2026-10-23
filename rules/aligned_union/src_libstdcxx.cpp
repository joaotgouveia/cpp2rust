#include <algorithm>
#include <type_traits>
#include <utility>

template <std::size_t T1, typename... T2>
using t1 = std::aligned_union<T1, T2...>;

template <std::size_t T1, typename... T2>
using t2 = typename std::aligned_union<T1, T2...>::type;

template <std::size_t T1, typename... T2> std::size_t f1() {
  return std::aligned_union<T1, T2...>::alignment_value;
}
