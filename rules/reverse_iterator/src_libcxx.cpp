#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f20(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f21(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 == a1;
}

template <typename T1>
std::reverse_iterator<T1>
f22(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2>
auto f23(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
bool f24(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f25(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f26(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f27(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return a0 >= a1;
}

template <typename T1>
using t9 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t10 = typename std::reverse_iterator<T1>::value_type;

template <typename T1> std::reverse_iterator<T1> f28(T1 a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1> T1 f29(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value,
                            int>::type T3 = 0>
std::reverse_iterator<T1> f30(const std::reverse_iterator<T2> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value &&
                                std::is_assignable<T1 &, const T2 &>::value,
                            int>::type T3 = 0>
std::reverse_iterator<T1> &f31(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
