#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f29(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f30(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f31(const std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f32(std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f33(const std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f34(std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <typename T1, typename T2>
auto f35(T1 &&a0, T2 &&a1)
    -> decltype(std::make_pair<T1, T2>(std::forward<T1>(a0),
                                       std::forward<T2>(a1))) {
  return std::make_pair<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}
