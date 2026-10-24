#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_convertible<const T3 &, T1>::value &&
                                 std::is_convertible<const T4 &, T2>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> f61(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_constructible<T1, const T3 &>::value &&
                                 std::is_constructible<T2, const T4 &>::value &&
                                 !(std::is_convertible<const T3 &, T1>::value &&
                                   std::is_convertible<const T4 &, T2>::value),
                             int>
              T5 = 0>
std::pair<T1, T2> f62(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_convertible<T3, T1>::value &&
                                 std::is_convertible<T4, T2>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> f63(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_constructible<T1, T3>::value &&
                             std::is_constructible<T2, T4>::value &&
                             !(std::is_convertible<T3, T1>::value &&
                               std::is_convertible<T4, T2>::value)>
              T5 = 0>
std::pair<T1, T2> f64(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_assignable<T1 &, const T3 &>::value &&
                                 std::is_assignable<T2 &, const T4 &>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> &f65(std::pair<T1, T2> &a0, const std::tuple<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_assignable<T1 &, T3 &&>::value &&
                                 std::is_assignable<T2 &, T4 &&>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> &f66(std::pair<T1, T2> &a0, std::tuple<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_convertible<const T3 &, T1>::value &&
                                 std::is_convertible<const T3 &, T2>::value,
                             int>
              T4 = 0>
std::pair<T1, T2> f67(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_constructible<T1, const T3 &>::value &&
                                 std::is_constructible<T2, const T3 &>::value &&
                                 !(std::is_convertible<const T3 &, T1>::value &&
                                   std::is_convertible<const T3 &, T2>::value),
                             int>
              T4 = 0>
std::pair<T1, T2> f68(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_convertible<T3, T1>::value &&
                                 std::is_convertible<T3, T2>::value,
                             int>
              T4 = 0>
std::pair<T1, T2> f69(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_constructible<T1, T3>::value &&
                                 std::is_constructible<T2, T3>::value &&
                                 !(std::is_convertible<T3, T1>::value &&
                                   std::is_convertible<T3, T2>::value),
                             int>
              T4 = 0>
std::pair<T1, T2> f70(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_assignable<T1 &, const T3 &>::value &&
                                 std::is_assignable<T2 &, const T3 &>::value,
                             int>
              T4 = 0>
std::pair<T1, T2> &f71(std::pair<T1, T2> &a0, const std::array<T3, 2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::is_assignable<T1 &, T3>::value &&
                                 std::is_assignable<T2 &, T3>::value,
                             int>
              T4 = 0>
std::pair<T1, T2> &f72(std::pair<T1, T2> &a0, std::array<T3, 2> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}
