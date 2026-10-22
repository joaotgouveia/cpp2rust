#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t3 = std::tuple<T1, T2>;

template <typename T1, typename T2>
std::tuple<T1, T2> f63(const std::tuple<T1, T2> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2>
std::tuple<T1, T2> f64(std::tuple<T1, T2> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f65(std::tuple<T1, T2> &a0, std::tuple<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::tuple<T1, T2> f66() {
  return std::tuple<T1, T2>();
}

template <typename T1, typename T2> std::tuple<T1, T2> f67() {
  return std::tuple<T1, T2>();
}

template <typename T1, typename T2>
std::tuple<T1, T2> f68(const T1 &a0, const T2 &a1) {
  return std::tuple<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::tuple<T1, T2> f69(const T1 &a0, const T2 &a1) {
  return std::tuple<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f70(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f71(T3 &&a0, T4 &&a1) {
  return std::tuple<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f72(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f73(const std::tuple<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f74(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f75(std::tuple<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f76(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f77(const std::pair<T3, T4> &a0) {
  return std::tuple<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f78(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> f79(std::pair<T3, T4> &&a0) {
  return std::tuple<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f80(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f81(std::allocator_arg_t a0, const T3 &a1) {
  return std::tuple<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f82(std::allocator_arg_t a0, const T3 &a1, const T1 &a2,
                       const T2 &a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f83(std::allocator_arg_t a0, const T3 &a1, const T1 &a2,
                       const T2 &a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f84(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f85(std::allocator_arg_t a0, const T3 &a1, T4 &&a2,
                       T5 &&a3) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::forward<T4>(a2),
                            std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f86(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T1, T2> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::tuple<T1, T2> f87(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T1, T2> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f88(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f89(std::allocator_arg_t a0, const T3 &a1,
                       const std::tuple<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f90(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f91(std::allocator_arg_t a0, const T3 &a1,
                       std::tuple<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f92(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f93(std::allocator_arg_t a0, const T3 &a1,
                       const std::pair<T4, T5> &a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f94(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::tuple<T1, T2> f95(std::allocator_arg_t a0, const T3 &a1,
                       std::pair<T4, T5> &&a2) {
  return std::tuple<T1, T2>(std::move(a0), a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f96(std::tuple<T1, T2> &a0, const std::tuple<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f97(std::tuple<T1, T2> &a0, std::tuple<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f98(std::tuple<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::tuple<T1, T2> &f99(std::tuple<T1, T2> &a0, std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
