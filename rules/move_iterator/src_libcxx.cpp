#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value,
                            int>::type T3 = 0>
std::move_iterator<T1> f28(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value &&
                                std::is_assignable<T1 &, const T2 &>::value,
                            int>::type T3 = 0>
std::move_iterator<T1> &f29(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
auto f30(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f32(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f34(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 >= a1;
}
