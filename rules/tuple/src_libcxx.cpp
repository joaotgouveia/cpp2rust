#include "../common/init.h"
#include "../common/traits.h"
#include <array>
#include <tuple>
#include <utility>

template <typename T1> std::tuple<> f114(std::array<T1, 0> a0) {
  return std::tuple<>(a0);
}

template <typename T1, typename T2>
std::tuple<> f115(std::allocator_arg_t a0, const T1 &a1, std::array<T2, 0> a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f116(const std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f117(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f118(std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f119(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f120(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f121(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2, std::size_t T3>
std::tuple<T1...> &f122(std::tuple<T1...> &a0, const std::array<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, std::size_t T3>
std::tuple<T1...> &f123(std::tuple<T1...> &a0, std::array<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

std::tuple<> f124() { return std::tuple_cat(); }

template <typename T1, typename... T2>
auto f125(T1 &&a0, T2 &&...a1)
    -> decltype(std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                          std::forward<T2>(a1)...)) {
  return std::tuple_cat<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}
