#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <type_traits>
#include <utility>
#include <variant>

using t1 = std::bad_variant_access;

std::bad_variant_access f1() { return std::bad_variant_access(); }

const char *f2(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f3(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f4(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f5(std::bad_variant_access &a0,
                            const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f6(std::bad_variant_access &a0,
                            std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}

template <std::size_t T1, typename... T2>
const std::variant_alternative_t<T1, std::variant<T2...>> &&
f7(const std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
std::variant_alternative_t<T1, std::variant<T2...>> &&
f8(std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
const std::variant_alternative_t<T1, std::variant<T2...>> &
f9(const std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::variant_alternative_t<T1, std::variant<T2...>> &
f10(std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2>
const T1 &&f11(const std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2> T1 &&f12(std::variant<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
const T1 &f13(const std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2> T1 &f14(std::variant<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::add_pointer_t<const std::variant_alternative_t<T1, std::variant<T2...>>>
f15(const std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
std::add_pointer_t<std::variant_alternative_t<T1, std::variant<T2...>>>
f16(std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::add_pointer_t<const T1> f17(const std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2>
std::add_pointer_t<T1> f18(std::variant<T2...> *a0) {
  return std::get_if<T1, T2...>(a0);
}

template <typename T1, typename... T2> bool f19(const std::variant<T2...> &a0) {
  return std::holds_alternative<T1, T2...>(a0);
}

template <typename... T1>
bool f20(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 != a1;
}

template <typename... T1>
bool f21(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 < a1;
}

template <typename... T1>
bool f22(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 <= a1;
}

template <typename... T1>
bool f23(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 == a1;
}

template <typename... T1>
bool f24(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 > a1;
}

template <typename... T1>
bool f25(const std::variant<T1...> &a0, const std::variant<T1...> &a1) {
  return a0 >= a1;
}

template <typename... T1>
void f26(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1> using t2 = std::variant<T1...>;

template <typename... T1>
std::variant<T1...> f27(const std::variant<T1...> &a0) {
  return std::variant<T1...>(a0);
}

template <typename... T1> std::variant<T1...> f28(std::variant<T1...> &&a0) {
  return std::variant<T1...>(std::move(a0));
}

template <typename... T1>
std::variant<T1...> &f29(std::variant<T1...> &a0,
                         const std::variant<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
std::variant<T1...> &f30(std::variant<T1...> &a0, std::variant<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> bool f31(const std::variant<T1...> &a0) {
  return a0.valueless_by_exception();
}

template <typename... T1> std::size_t f32(const std::variant<T1...> &a0) {
  return a0.index();
}

template <typename... T1>
void f33(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return a0.swap(a1);
}

template <typename... T1> std::variant<T1...> f34() {
  return std::variant<T1...>();
}

template <typename... T1, typename T2> std::variant<T1...> f35(T2 &&a0) {
  return std::variant<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2, typename... T3>
std::variant<T1...> f36(std::in_place_type_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}

template <typename... T1, typename T2, typename T3, typename... T4>
std::variant<T1...> f37(std::in_place_type_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(a0, a1, std::forward<T4>(a2)...);
}

template <typename... T1, std::size_t T2, typename... T3>
std::variant<T1...> f38(std::in_place_index_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}

template <typename... T1, std::size_t T2, typename T3, typename... T4>
std::variant<T1...> f39(std::in_place_index_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(a0, a1, std::forward<T4>(a2)...);
}

template <typename... T1, typename T2>
std::variant<T1...> &f40(std::variant<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2, typename... T3>
T2 &f41(std::variant<T1...> &a0, T3 &&...a1) {
  return a0.template emplace<T2, T3...>(std::forward<T3>(a1)...);
}

template <typename... T1, typename T2, typename T3, typename... T4>
T2 &f42(std::variant<T1...> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T2, T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <std::size_t T1, typename T2>
using t3 = std::variant_alternative_t<T1, T2>;

template <typename T1, typename... T2> auto f43(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
