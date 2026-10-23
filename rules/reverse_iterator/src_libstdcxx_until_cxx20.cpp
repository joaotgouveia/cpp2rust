#include <algorithm>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::reverse_iterator<T1> f24(const std::reverse_iterator<T2> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::reverse_iterator<T1> &f25(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
