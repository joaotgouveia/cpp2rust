#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1>
using t7 = typename std::move_iterator<T1>::iterator_category;

template <typename T1> T1 f6(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::reference
f7(const std::move_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::move_iterator<T1> f8(const std::move_iterator<T1> &a0,
                          typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator+(std::move(a1));
}

template <typename T1>
std::move_iterator<T1> &
f9(std::move_iterator<T1> &a0,
   typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator+=(std::move(a1));
}

template <typename T1>
std::move_iterator<T1>
f10(const std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator-(std::move(a1));
}

template <typename T1>
std::move_iterator<T1> &
f11(std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator-=(std::move(a1));
}

template <typename T1>
typename std::move_iterator<T1>::reference
f12(const std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator[](std::move(a1));
}
