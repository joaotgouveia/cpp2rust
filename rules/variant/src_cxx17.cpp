#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

template <std::size_t T1, typename... T2>
const std::variant_alternative_t<T1, std::variant<T2...>> &&
f1(const std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
std::variant_alternative_t<T1, std::variant<T2...>> &&
f2(std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
const std::variant_alternative_t<T1, std::variant<T2...>> &
f3(const std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::variant_alternative_t<T1, std::variant<T2...>> &
f4(std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2>
const T1 &&f5(const std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2> T1 &&f6(std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
const T1 &f7(const std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2> T1 &f8(std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::add_pointer_t<const std::variant_alternative_t<T1, std::variant<T2...>>>
f9(const std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::add_pointer_t<std::variant_alternative_t<T1, std::variant<T2...>>>
f10(std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::add_pointer_t<const T1> f11(const std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::add_pointer_t<T1> f12(std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2> bool f13(const std::variant<T2...> &a0) {
  return std::holds_alternative<T1, T2...>(a0);
}

template <typename... T1>
bool f14(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 != a1;
}

template <typename... T1>
bool f15(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 < a1;
}

template <typename... T1>
bool f16(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 <= a1;
}

template <typename... T1>
bool f17(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 == a1;
}

template <typename... T1>
bool f18(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 > a1;
}

template <typename... T1>
bool f19(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 >= a1;
}

template <typename... T1> using t1 = std::variant<T1...>;

template <typename... T1>
std::variant<T1...> f20(const std::variant<T1...> &a0) {
  return std::variant<T1...>(a0);
}

template <typename... T1> std::variant<T1...> f21(std::variant<T1...> &&a0) {
  return std::variant<T1...>(std::move(a0));
}

template <typename... T1>
std::variant<T1...> &f22(std::variant<T1...> &a0,
                         const std::variant<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
std::variant<T1...> &f23(std::variant<T1...> &a0, std::variant<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> bool f24(const std::variant<T1...> &a0) {
  return a0.valueless_by_exception();
}

template <typename... T1> std::size_t f25(const std::variant<T1...> &a0) {
  return a0.index();
}

template <typename... T1>
void f26(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return a0.swap(a1);
}

template <typename... T1> std::variant<T1...> f27() {
  return std::variant<T1...>();
}

template <typename... T1, typename T2> std::variant<T1...> f28(T2 &&a0) {
  return std::variant<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2, typename... T3>
std::variant<T1...> f29(std::in_place_type_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}

template <typename... T1, typename T2, typename T3, typename... T4>
std::variant<T1...> f30(std::in_place_type_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(a0, a1, std::forward<T4>(a2)...);
}

template <typename... T1, std::size_t T2, typename... T3>
std::variant<T1...> f31(std::in_place_index_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}

template <typename... T1, std::size_t T2, typename T3, typename... T4>
std::variant<T1...> f32(std::in_place_index_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(a0, a1, std::forward<T4>(a2)...);
}

template <typename... T1, typename T2>
std::variant<T1...> &f33(std::variant<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2, typename... T3>
T2 &f34(std::variant<T1...> &a0, T3 &&...a1) {
  return a0.template emplace<T2, T3...>(std::forward<T3>(a1)...);
}

template <typename... T1, typename T2, typename T3, typename... T4>
T2 &f35(std::variant<T1...> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T2, T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <std::size_t T1, typename T2>
using t2 = std::variant_alternative_t<T1, T2>;
