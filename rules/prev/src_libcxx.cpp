#include <iterator>
#include <utility>

template <typename T1,
          std::__enable_if_t<std::__has_input_iterator_category<T1>::value, int>
              T2 = 0>
T1 f1(T1 a0) {
  return std::prev<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::__has_input_iterator_category<T1>::value, int>
              T2 = 0>
T1 f2(T1 a0, typename std::iterator_traits<T1>::difference_type a1) {
  return std::prev<T1>(a0, a1);
}
