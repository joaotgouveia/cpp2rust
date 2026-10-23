#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f21(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 != a1;
}

template <typename T1>
typename std::move_iterator<T1>::pointer f22(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 f23(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
std::move_iterator<T1> f24(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value,
                            int>::type T3 = 0>
std::move_iterator<T1> f25(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<!std::is_same<T2, T1>::value &&
                                std::is_convertible<const T2 &, T1>::value &&
                                std::is_assignable<T1 &, const T2 &>::value,
                            int>::type T3 = 0>
std::move_iterator<T1> &f26(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
std::move_iterator<T1> f27(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return a0 + a1;
}
