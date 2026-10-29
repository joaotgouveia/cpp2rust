#include <algorithm>
#include <utility>

template <typename T1, T1... T2> using t1 = std::integer_sequence<T1, T2...>;

template <typename T1, T1... T2>
using t2 = typename std::integer_sequence<T1, T2...>::value_type;

template <typename T1, T1... T2> std::size_t f1() {
  return std::integer_sequence<T1, T2...>::size();
}
