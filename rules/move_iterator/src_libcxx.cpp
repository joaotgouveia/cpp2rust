#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f24(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 == a1;
}

template <typename T1> std::move_iterator<T1> f25(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f26(T1 a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1> T1 f27(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
auto f28(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
bool f29(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f30(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f32(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 >= a1;
}
