#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1> std::tuple<T1 &&...> f1(T1 &&...a0) {
  return std::forward_as_tuple<T1...>(std::forward<T1>(a0)...);
}

template <std::size_t T1, typename... T2>
const typename std::tuple_element<T1, std::tuple<T2...>>::type &&
f2(const std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
typename std::tuple_element<T1, std::tuple<T2...>>::type &&
f3(std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
const typename std::tuple_element<T1, std::tuple<T2...>>::type &
f4(const std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
typename std::tuple_element<T1, std::tuple<T2...>>::type &
f5(std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename... T1>
auto f6(T1 &&...a0)
    -> decltype(std::make_tuple<T1...>(std::forward<T1>(a0)...)) {
  return std::make_tuple<T1...>(std::forward<T1>(a0)...);
}

template <typename... T1, typename... T2>
bool f7(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 == a1;
}

template <typename... T1>
void f8(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1> std::tuple<T1 &...> f9(T1 &...a0) {
  return std::tie<T1...>(a0...);
}

using t1 = std::tuple<>;

void f10(std::tuple<> &a0, std::tuple<> &a1) { return a0.swap(a1); }

std::tuple<> f11() { return std::tuple<>(); }

std::tuple<> f12(const std::tuple<> &a0) { return std::tuple<>(a0); }

std::tuple<> f13(std::tuple<> &&a0) { return std::tuple<>(std::move(a0)); }

std::tuple<> &f14(std::tuple<> &a0, const std::tuple<> &a1) {
  return a0.operator=(a1);
}

std::tuple<> &f15(std::tuple<> &a0, std::tuple<> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::tuple<> f16(std::allocator_arg_t a0, const T1 &a1) {
  return std::tuple<>(std::move(a0), a1);
}

template <typename T1>
std::tuple<> f17(std::allocator_arg_t a0, const T1 &a1,
                 const std::tuple<> &a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

template <typename... T1> using t2 = std::tuple<T1...>;

template <typename... T1> std::tuple<T1...> f18(const std::tuple<T1...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1> std::tuple<T1...> f19(std::tuple<T1...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1>
void f20(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return a0.swap(a1);
}

template <typename... T1> std::tuple<T1...> f21() {
  return std::tuple<T1...>();
}

template <typename... T1> std::tuple<T1...> f22(const T1 &...a0) {
  return std::tuple<T1...>(a0...);
}

template <typename... T1, typename... T2> std::tuple<T1...> f23(T2 &&...a0) {
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f24(const std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f25(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f26(std::allocator_arg_t a0, const T2 &a1) {
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f27(std::allocator_arg_t a0, const T2 &a1, const T1 &...a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2...);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f28(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename... T1, typename T2>
std::tuple<T1...> f29(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T1...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2>
std::tuple<T1...> f30(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T1...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f31(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f32(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f33(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f34(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1>
auto f35(T1 &&...a0)
    -> decltype(std::tuple_cat<T1...>(std::forward<T1>(a0)...)) {
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
