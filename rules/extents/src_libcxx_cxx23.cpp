#include <array>
#include <cstddef>
#include <mdspan>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2> using t1 = std::dextents<T1, T2>;

template <typename T1, std::size_t... T2> using t2 = std::extents<T1, T2...>;

template <typename T1, std::size_t... T2>
using t3 = typename std::extents<T1, T2...>::index_type;

template <typename T1, std::size_t... T2>
using t4 = typename std::extents<T1, T2...>::size_type;

template <typename T1, std::size_t... T2>
using t5 = typename std::extents<T1, T2...>::rank_type;

template <typename T1, std::size_t... T2>
typename std::extents<T1, T2...>::rank_type f1() {
  return std::extents<T1, T2...>::rank();
}

template <typename T1, std::size_t... T2>
typename std::extents<T1, T2...>::rank_type f2() {
  return std::extents<T1, T2...>::rank_dynamic();
}

template <typename T1, std::size_t... T2>
typename std::extents<T1, T2...>::index_type
f3(const std::extents<T1, T2...> &a0,
   typename std::extents<T1, T2...>::rank_type a1) {
  return a0.extent(a1);
}

template <typename T1, std::size_t... T2>
std::size_t f4(typename std::extents<T1, T2...>::rank_type a0) {
  return std::extents<T1, T2...>::static_extent(a0);
}

template <typename T1, std::size_t... T2> std::extents<T1, T2...> f5() {
  return std::extents<T1, T2...>();
}

template <typename T1, std::size_t... T2, typename... T3>
std::extents<T1, T2...> f6(T3... a0) {
  return std::extents<T1, T2...>(a0...);
}

template <typename T1, std::size_t... T2, typename T3, std::size_t T4>
std::extents<T1, T2...> f7(const std::array<T3, T4> &a0) {
  return std::extents<T1, T2...>(a0);
}

template <typename T1, std::size_t... T2, typename T3, std::size_t T4>
std::extents<T1, T2...> f8(const std::span<T3, T4> &a0) {
  return std::extents<T1, T2...>(a0);
}

template <typename T1, std::size_t... T2, typename T3, std::size_t... T4>
std::extents<T1, T2...> f9(const std::extents<T3, T4...> &a0) {
  return std::extents<T1, T2...>(a0);
}
