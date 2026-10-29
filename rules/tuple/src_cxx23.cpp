#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename T2> decltype(auto) f1(T1 &&a0, T2 &&a1) {
  return std::apply<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename... T1> std::tuple<T1 &&...> f2(T1 &&...a0) {
  return std::forward_as_tuple<T1...>(std::forward<T1>(a0)...);
}

template <std::size_t T1, typename... T2>
const typename std::tuple_element<T1, std::tuple<T2...>>::type &&
f3(const std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
typename std::tuple_element<T1, std::tuple<T2...>>::type &&
f4(std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <std::size_t T1, typename... T2>
const typename std::tuple_element<T1, std::tuple<T2...>>::type &
f5(const std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <std::size_t T1, typename... T2>
typename std::tuple_element<T1, std::tuple<T2...>>::type &
f6(std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2>
const T1 &&f7(const std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2> T1 &&f8(std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
const T1 &f9(const std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2> T1 &f10(std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename T2> T1 f11(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}

template <typename... T1> auto f12(T1 &&...a0) {
  return std::make_tuple<T1...>(std::forward<T1>(a0)...);
}

template <typename... T1, typename... T2>
auto f13(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return std::operator<=> <T1...>(a0, a1);
}

template <typename... T1, typename... T2>
bool f14(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return std::operator== <T1...>(a0, a1);
}

template <typename... T1>
void f15(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return std::swap<T1...>(a0, a1);
}

template <typename... T1>
typename std::enable_if<std::conjunction<std::is_swappable<T1>...>::value>::type
f16(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1> std::tuple<T1 &...> f17(T1 &...a0) {
  return std::tie<T1...>(a0...);
}

using t1 = std::tuple<>;

void f18(std::tuple<> &a0, std::tuple<> &a1) { return a0.swap(a1); }

void f19(const std::tuple<> &a0, const std::tuple<> &a1) { return a0.swap(a1); }

std::tuple<> f20() { return std::tuple<>(); }

std::tuple<> f21(const std::tuple<> &a0) { return std::tuple<>(a0); }

std::tuple<> f22(std::tuple<> &&a0) { return std::tuple<>(std::move(a0)); }

std::tuple<> &f23(std::tuple<> &a0, const std::tuple<> &a1) {
  return a0.operator=(a1);
}

std::tuple<> &f24(std::tuple<> &a0, std::tuple<> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::tuple<> f25(std::allocator_arg_t a0, const T1 &a1) {
  return std::tuple<>(std::move(a0), a1);
}

template <typename T1>
std::tuple<> f26(std::allocator_arg_t a0, const T1 &a1,
                 const std::tuple<> &a2) {
  return std::tuple<>(std::move(a0), a1, a2);
}

template <typename... T1> using t2 = std::tuple<T1...>;

template <typename... T1>
std::tuple<T1...> f27()
  requires((std::is_default_constructible_v<T1> && ...))
{
  return std::tuple<T1...>();
}

template <typename... T1> std::tuple<T1...> f28(const std::tuple<T1...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1> std::tuple<T1...> f29(std::tuple<T1...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1>
std::tuple<T1...> &f30(std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
std::tuple<T1...> &f31(std::tuple<T1...> &a0, std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
const std::tuple<T1...> &f32(const std::tuple<T1...> &a0,
                             const std::tuple<T1...> &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f33(const std::tuple<T1...> &a0,
                             std::tuple<T1...> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f34(std::tuple<T1...> &a0, std::tuple<T1...> &a1) {
  return a0.swap(a1);
}

template <typename... T1>
void f35(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return a0.swap(a1);
}

template <typename... T1, typename T2 = void>
std::tuple<T1...> f36(const T1 &...a0) {
  return std::tuple<T1...>(a0...);
}

template <typename... T1, typename... T2> std::tuple<T1...> f37(T2 &&...a0) {
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f38(const std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f39(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename... T2>
std::tuple<T1...> f40(std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f41(const std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f42(const std::pair<T2, T3> &a0)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f43(std::pair<T2, T3> &&a0)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f44(std::pair<T2, T3> &a0)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f45(const std::pair<T2, T3> &&a0)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f46(T2 &&a0)
  requires(!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return std::tuple<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f47(std::allocator_arg_t a0, const T2 &a1)
  requires((std::is_default_constructible_v<T1> && ...))
{
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f48(std::allocator_arg_t a0, const T2 &a1, const T1 &...a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2...);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f49(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename... T1, typename T2>
std::tuple<T1...> f50(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T1...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2>
std::tuple<T1...> f51(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T1...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f52(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f53(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f54(std::allocator_arg_t a0, const T2 &a1,
                      std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f55(std::allocator_arg_t a0, const T2 &a1,
                      const std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f56(std::allocator_arg_t a0, const T2 &a1,
                      const std::pair<T3, T4> &a2)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f57(std::allocator_arg_t a0, const T2 &a1,
                      std::pair<T3, T4> &&a2)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f58(std::allocator_arg_t a0, const T2 &a1,
                      std::pair<T3, T4> &a2)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f59(std::allocator_arg_t a0, const T2 &a1,
                      const std::pair<T3, T4> &&a2)
  requires(sizeof...(T1) == 2)
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f60(std::allocator_arg_t a0, const T2 &a1, T3 &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2));
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f61(std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f62(std::tuple<T1...> &a0, std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f63(const std::tuple<T1...> &a0,
                             const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f64(const std::tuple<T1...> &a0,
                             std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f65(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f66(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f67(const std::tuple<T1...> &a0,
                             const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f68(const std::tuple<T1...> &a0,
                             std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2>
std::tuple<T1...> &f69(std::tuple<T1...> &a0, T2 &&a1)
  requires(!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
const std::tuple<T1...> &f70(const std::tuple<T1...> &a0, T2 &&a1)
  requires(!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
bool f71(const std::tuple<T1...> &a0, const T2 &a1) {
  return operator==(a0, a1);
}

template <typename... T1> auto f72(T1 &&...a0) {
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
