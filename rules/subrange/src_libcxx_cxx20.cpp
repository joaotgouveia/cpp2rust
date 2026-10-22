#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::ranges::borrowed_subrange_t<T1>;

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f25(std::ranges::subrange<T2, T3, T4> &&a0) {
  return std::ranges::get<T1, T2, T3, T4>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f26(const std::ranges::subrange<T2, T3, T4> &a0) {
  return std::ranges::get<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::make_unsigned_t<std::iter_difference_t<T1>>
f27(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f28(T4 a0, T2 a1, std::make_unsigned_t<std::iter_difference_t<T1>> a2) {
  return std::ranges::subrange<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3>
f29(T4 &&a0, std::make_unsigned_t<std::iter_difference_t<T1>> a1) {
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0), a1);
}

using t3 = std::ranges::subrange_kind;

std::ranges::subrange_kind f30() { return std::ranges::subrange_kind::unsized; }

std::ranges::subrange_kind f31() { return std::ranges::subrange_kind::sized; }
