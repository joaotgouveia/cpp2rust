#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              !std::__has_exactly_input_iterator_category<T1>::value, int>
              T4 = 0>
T3 f1(T1 a0, T2 a1, T3 a2) {
  return std::copy_n<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T1>::value, int>
              T4 = 0>
T3 f2(T1 a0, T2 a1, T3 a2) {
  return std::copy_n<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}
