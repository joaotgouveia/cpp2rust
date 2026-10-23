#include <algorithm>
#include <utility>

template <typename T1>
std::move_iterator<T1> f13(typename std::move_iterator<T1>::iterator_type a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1>
typename std::move_iterator<T1>::pointer f14(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::move_iterator<T1> f15(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f16(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f17(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
