#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f1(const std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f2(std::pair<T2, T3> &&a0) {
  return std::get<T1, T2, T3>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f3(const std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f4(std::pair<T2, T3> &a0) {
  return std::get<T1, T2, T3>(a0);
}

template <typename T1, typename T2>
auto f5(T1 &&a0, T2 &&a1)
    -> decltype(std::make_pair<T1, T2>(std::forward<T1>(a0),
                                       std::forward<T2>(a1))) {
  return std::make_pair<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

using t1 = std::piecewise_construct_t;

std::piecewise_construct_t f6() { return std::piecewise_construct_t(); }

std::piecewise_construct_t f7(const std::piecewise_construct_t &a0) {
  return std::piecewise_construct_t(a0);
}

std::piecewise_construct_t f8(std::piecewise_construct_t &&a0) {
  return std::piecewise_construct_t(std::move(a0));
}

std::piecewise_construct_t &f9(std::piecewise_construct_t &a0,
                               const std::piecewise_construct_t &a1) {
  return a0.operator=(a1);
}

std::piecewise_construct_t &f10(std::piecewise_construct_t &a0,
                                std::piecewise_construct_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f11(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
