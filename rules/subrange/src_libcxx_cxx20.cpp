#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::ranges::borrowed_subrange_t<T1>;

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f28(std::ranges::subrange<T2, T3, T4> &&a0) {
  return std::ranges::get<T1, T2, T3, T4>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f29(const std::ranges::subrange<T2, T3, T4> &a0) {
  return std::ranges::get<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::make_unsigned_t<std::iter_difference_t<T1>>
f30(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f31(T4 a0, T2 a1, std::make_unsigned_t<std::iter_difference_t<T1>> a2) {
  return std::ranges::subrange<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f32(T4 &&a0, std::make_unsigned_t<std::iter_difference_t<T1>> a1) {
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0), a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
bool f33(std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
bool f34(const std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
auto f35(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
auto f36(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f37(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f38(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f39(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3,
          typename T4 = std::ranges::subrange<T1, T2, T3>>
decltype(auto) f40(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}

using t3 = std::ranges::subrange_kind;

std::ranges::subrange_kind f41() { return std::ranges::subrange_kind::unsized; }

std::ranges::subrange_kind f42() { return std::ranges::subrange_kind::sized; }
