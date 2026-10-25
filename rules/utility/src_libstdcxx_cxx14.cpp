#include <algorithm>
#include <utility>

template <typename T1, typename T2 = T1> T1 f7(T1 &a0, T2 &&a1) {
  return std::exchange<T1, T2>(a0, std::forward<T2>(a1));
}

template <std::size_t... T1> using t2 = std::index_sequence<T1...>;

template <typename... T1> using t3 = std::index_sequence_for<T1...>;

template <std::size_t T1> using t4 = std::make_index_sequence<T1>;

template <typename T1, T1 T2> using t5 = std::make_integer_sequence<T1, T2>;

template <std::size_t T1, typename T2> using t6 = std::tuple_element_t<T1, T2>;
