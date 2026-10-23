#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f39(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<
              std::__is_swappable_v<T1> && std::__is_swappable_v<T2>, int>
              T3 = 0>
void f40(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f41(const std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f42(std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f43(const std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f44(std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <typename T1, typename T2>
auto f45(T1 &&a0, T2 &&a1)
    -> decltype(std::make_pair<T1, T2>(std::forward<T1>(a0),
                                       std::forward<T2>(a1))) {
  return std::make_pair<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3, T4>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f46(T3 &&a0, T4 &&a1) {
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__check_pair_construction<T1, T2>::
                  template __is_pair_constructible<const T3 &, const T4 &>(),
              int>
              T5 = 0>
std::pair<T1, T2> f47(const std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3, T4>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f48(std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_assignable<T1 &, const T3 &>::value &&
                                 std::is_assignable<T2 &, const T4 &>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> &f49(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::is_assignable<T1 &, T3>::value &&
                                 std::is_assignable<T2 &, T4>::value,
                             int>
              T5 = 0>
std::pair<T1, T2> &f50(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
