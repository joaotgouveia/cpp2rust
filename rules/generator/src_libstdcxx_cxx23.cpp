#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <generator>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::generator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::generator<T1, T2, T3>::yielded;

template <typename T1, typename T2, typename T3>
using t3 = typename std::generator<T1, T2, T3>::promise_type;

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> f1(std::generator<T1, T2, T3> &&a0) {
  return std::generator<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> &f2(std::generator<T1, T2, T3> &a0,
                               std::generator<T1, T2, T3> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t f3(const std::generator<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
bool f4(std::generator<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f5(const std::generator<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f6(std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f7(const std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
auto f8(std::generator<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f9(const std::generator<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f10(std::generator<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
auto f11(const std::generator<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f12(std::generator<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f13(const std::generator<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f14(std::generator<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f15(const std::generator<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
auto f16(std::generator<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f17(const std::generator<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f18(std::generator<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f19(const std::generator<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2> using t4 = std::pmr::generator<T1, T2>;
