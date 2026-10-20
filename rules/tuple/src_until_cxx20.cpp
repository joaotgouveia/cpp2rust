#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2>
bool f36(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 != a1;
}

template <typename... T1, typename... T2>
bool f37(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 < a1;
}

template <typename... T1, typename... T2>
bool f38(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 <= a1;
}

template <typename... T1, typename... T2>
bool f39(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 > a1;
}

template <typename... T1, typename... T2>
bool f40(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2> using t3 = std::tuple<T1, T2>;

template <typename T1, typename T2>
std::tuple<T1, T2> f41(const std::tuple<T1, T2> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2>
std::tuple<T1, T2> f42(std::tuple<T1, T2> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f43(std::tuple<T1, T2> &a0, std::tuple<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::tuple<T1, T2> f44() {
  return std::tuple<T1, T2>();
}

template <typename T1, typename T2> std::tuple<T1, T2> f45() {
  return std::tuple<T1, T2>();
}

template <typename T1, typename T2>
std::tuple<T1, T2> f46(const T1 &a0, const T2 &a1) {
  return std::tuple<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::tuple<T1, T2> f47(const T1 &a0, const T2 &a1) {
  return std::tuple<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f48(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f49(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f50(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f51(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f52(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f53(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f54(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f55(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f56(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f57(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f58(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f59(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f60(std::allocator_arg_t a0, const T3 &a1, const T1 &a2,
                       const T2 &a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f61(std::allocator_arg_t a0, const T3 &a1, const T1 &a2,
                       const T2 &a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f62(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f63(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f64(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T1, T2> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f65(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T1, T2> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f66(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f67(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f68(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f69(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f70(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f71(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f72(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f73(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f74(std::tuple<T1, T2> &a0, const std::tuple<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f75(std::tuple<T1, T2> &a0, std::tuple<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f76(std::tuple<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f77(std::tuple<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
