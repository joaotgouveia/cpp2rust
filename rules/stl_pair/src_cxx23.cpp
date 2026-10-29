#include <algorithm>
#include <utility>

template <typename T1, typename T2>
const T1 &&f1(const std::pair<T1, T2> &&a0) {
  return std::get(std::move(a0));
}

template <typename T1, typename T2>
const T1 &&f2(const std::pair<T2, T1> &&a0) {
  return std::get(std::move(a0));
}

template <typename T1, typename T2> T1 &&f3(std::pair<T1, T2> &&a0) {
  return std::get(std::move(a0));
}

template <typename T1, typename T2> T1 &&f4(std::pair<T2, T1> &&a0) {
  return std::get(std::move(a0));
}

template <typename T1, typename T2> const T1 &f5(const std::pair<T1, T2> &a0) {
  return std::get(a0);
}

template <typename T1, typename T2> const T1 &f6(const std::pair<T2, T1> &a0) {
  return std::get(a0);
}

template <typename T1, typename T2> T1 &f7(std::pair<T1, T2> &a0) {
  return std::get(a0);
}

template <typename T1, typename T2> T1 &f8(std::pair<T2, T1> &a0) {
  return std::get(a0);
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f9(const std::pair<T2, T3> &&a0) {
  return std::get<T1>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &&
f10(std::pair<T2, T3> &&a0) {
  return std::get<T1>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3>
const typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f11(const std::pair<T2, T3> &a0) {
  return std::get<T1>(a0);
}

template <std::size_t T1, typename T2, typename T3>
typename std::tuple_element<T1, std::pair<T2, T3>>::type &
f12(std::pair<T2, T3> &a0) {
  return std::get<T1>(a0);
}

template <typename T1, typename T2> auto f13(T1 &&a0, T2 &&a1) {
  return std::make_pair(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f14(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f15(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator==(a0, a1);
}

const std::piecewise_construct_t &f16() { return std::piecewise_construct; }

using t1 = std::piecewise_construct_t;

std::piecewise_construct_t f17() { return std::piecewise_construct_t(); }

std::piecewise_construct_t f18(const std::piecewise_construct_t &a0) {
  return std::piecewise_construct_t(a0);
}

std::piecewise_construct_t f19(std::piecewise_construct_t &&a0) {
  return std::piecewise_construct_t(std::move(a0));
}

std::piecewise_construct_t &f20(std::piecewise_construct_t &a0,
                                const std::piecewise_construct_t &a1) {
  return a0.operator=(a1);
}

std::piecewise_construct_t &f21(std::piecewise_construct_t &a0,
                                std::piecewise_construct_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f22(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires(std::is_swappable_v<const T1> && std::is_swappable_v<const T2>)
{
  return std::swap(a0, a1);
}

template <typename T1, typename T2>
typename std::enable_if<
    std::integral_constant<bool, std::is_swappable<T1>::value &&
                                     std::is_swappable<T2>::value>::value>::type
f23(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return std::swap(a0, a1);
}
