#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f31(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f32(const std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f33(std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f34(const std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f35(std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <typename T1, typename T2>
auto f36(T1 &&a0, T2 &&a1)
    -> decltype(std::make_pair<T1, T2>(std::forward<T1>(a0),
                                       std::forward<T2>(a1))) {
  return std::make_pair<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename std::enable_if<std::is_assignable<T1 &, const T3 &>::value &&
                                std::is_assignable<T2 &, const T4 &>::value,
                            int>::type T5 = 0>
std::pair<T1, T2> &f37(std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<std::is_assignable<T1 &, T3>::value &&
                                      std::is_assignable<T2 &, T4>::value,
                                  int>::type T5 = 0>
std::pair<T1, T2> &f38(std::pair<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
