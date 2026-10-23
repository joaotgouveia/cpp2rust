#include <algorithm>
#include <utility>

template <typename T1>
std::reverse_iterator<T1>
f14(typename std::reverse_iterator<T1>::iterator_type a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> f15(const std::reverse_iterator<T1> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> &f16(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::reverse_iterator<T1>::iterator_type
f17(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::reverse_iterator<T1> f18(const std::reverse_iterator<T2> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::reverse_iterator<T1> &f19(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
