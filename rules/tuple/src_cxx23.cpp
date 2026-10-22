#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1>
void f46(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

void f47(const std::tuple<> &a0, const std::tuple<> &a1) { return a0.swap(a1); }

template <typename... T1>
const std::tuple<T1...> &f48(const std::tuple<T1...> &a0,
                             const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f49(const std::tuple<T1...> &a0,
                             std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f50(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.swap(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f51(std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f52(const std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f53(std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f54(const std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f55(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f56(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f57(std::allocator_arg_t a0, const T2 &a1,
                      std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f58(std::allocator_arg_t a0, const T2 &a1,
                      const std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f59(const std::tuple<T1...> &a0,
                             const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f60(const std::tuple<T1...> &a0,
                             std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f61(const std::tuple<T1...> &a0,
                             const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f62(const std::tuple<T1...> &a0,
                             std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
