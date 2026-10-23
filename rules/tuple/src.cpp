#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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

template <typename... T1> std::tuple<T1 &...> f8(T1 &...a0) {
  return std::tie<T1...>(a0...);
}

using t1 = std::tuple<>;

void f9(std::tuple<> &a0, std::tuple<> &a1) { return a0.swap(a1); }

std::tuple<> f10() { return std::tuple<>(); }

std::tuple<> f11(const std::tuple<> &a0) { return std::tuple<>(a0); }

std::tuple<> f12(std::tuple<> &&a0) { return std::tuple<>(std::move(a0)); }

std::tuple<> &f13(std::tuple<> &a0, const std::tuple<> &a1) {
  return a0.operator=(a1);
}

std::tuple<> &f14(std::tuple<> &a0, std::tuple<> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::tuple<> f15(std::allocator_arg_t a0, const T1 &a1) {
  return std::tuple<>(std::move(a0), a1);
}

template <typename T1>
std::tuple<> f16(std::allocator_arg_t a0, const T1 &a1,
                 const std::tuple<> &a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

template <typename... T1> using t2 = std::tuple<T1...>;

template <typename... T1> std::tuple<T1...> f17(const std::tuple<T1...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1> std::tuple<T1...> f18(std::tuple<T1...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1>
void f19(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return a0.swap(a1);
}
