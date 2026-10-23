#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1> std::tuple<T1...> f71() {
  return std::tuple<T1...>();
}

template <typename... T1>
std::tuple<T1...> &f72(std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
std::tuple<T1...> &f73(std::tuple<T1...> &a0, std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1, typename T2 = void>
std::tuple<T1...> f74(const T1 &...a0) {
  return std::tuple<T1...>(a0...);
}

template <typename... T1, typename... T2> std::tuple<T1...> f75(T2 &&...a0) {
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f76(const std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f77(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f78(const std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f79(std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f80(std::allocator_arg_t a0, const T2 &a1) {
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f81(std::allocator_arg_t a0, const T2 &a1, const T1 &...a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2...);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f82(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f83(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f84(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f85(std::allocator_arg_t a0, const T2 &a1,
                      const std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f86(std::allocator_arg_t a0, const T2 &a1,
                      std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f87(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f88(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f89(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f90(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
