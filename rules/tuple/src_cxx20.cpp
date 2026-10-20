#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2>
auto f84(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 <=> a1;
}

template <typename... T1>
std::tuple<T1...> &f85(std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
std::tuple<T1...> &f86(std::tuple<T1...> &a0, std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f87(const std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f88(std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f89(std::allocator_arg_t a0, const T2 &a1,
                      const std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f90(std::allocator_arg_t a0, const T2 &a1,
                      std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f91(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f92(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
