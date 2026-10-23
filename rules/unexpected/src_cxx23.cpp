#include <algorithm>
#include <expected>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::unexpected<T1>;

template <typename T1> std::unexpected<T1> f1(const std::unexpected<T1> &a0) {
  return std::unexpected<T1>(a0);
}

template <typename T1> std::unexpected<T1> f2(std::unexpected<T1> &&a0) {
  return std::unexpected<T1>(std::move(a0));
}

template <typename T1>
std::unexpected<T1> &f3(std::unexpected<T1> &a0,
                        const std::unexpected<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::unexpected<T1> &f4(std::unexpected<T1> &a0, std::unexpected<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> const T1 &f5(const std::unexpected<T1> &a0) {
  return a0.error();
}

template <typename T1> T1 &f6(std::unexpected<T1> &a0) { return a0.error(); }

template <typename T1> const T1 &&f7(const std::unexpected<T1> &&a0) {
  return std::move(a0).error();
}

template <typename T1> T1 &&f8(std::unexpected<T1> &&a0) {
  return std::move(a0).error();
}

template <typename T1>
void f9(std::unexpected<T1> &a0, std::unexpected<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::unexpected<T1> f10(T2 &&a0) {
  return std::unexpected<T1>(std::forward<T2>(a0));
}

template <typename T1, typename... T2>
std::unexpected<T1> f11(std::in_place_t a0, T2 &&...a1) {
  return std::unexpected<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::unexpected<T1> f12(std::in_place_t a0, std::initializer_list<T2> a1,
                        T3 &&...a2) {
  return std::unexpected<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1>
void f13(std::unexpected<T1> &a0, std::unexpected<T1> &a1) {
  return swap(a0, a1);
}
