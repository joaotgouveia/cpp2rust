#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1>
typename std::reverse_iterator<T1>::reference
f9(const std::reverse_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::reverse_iterator<T1>
f10(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::reverse_iterator<T1> &
f11(std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::reverse_iterator<T1>
f12(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::reverse_iterator<T1> &
f13(std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::reverse_iterator<T1>::reference
f14(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator[](a1);
}
