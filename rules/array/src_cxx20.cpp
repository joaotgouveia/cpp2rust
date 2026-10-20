#include "../common/init.h"
#include "../common/traits.h"
#include <array>
#include <type_traits>
#include <utility>

template <typename T1, std::size_t T2>
using t21 = typename std::array<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, std::size_t T2>
using t22 = typename std::array<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t23 = typename std::array<T1, T2>::reverse_iterator::value_type;

template <typename T1, std::size_t T2>
using t24 =
    typename std::array<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, std::size_t T2>
using t25 =
    typename std::array<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t26 = typename std::array<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, std::size_t T2>
auto f39(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return a0 <=> a1;
}

template <typename T1, std::size_t T2>
std::array<std::remove_cv_t<T1>, T2> f40(T1 (&&a0)[T2]) {
  return std::to_array<T1, T2>(std::move(a0));
}

template <typename T1, std::size_t T2>
std::array<std::remove_cv_t<T1>, T2> f41(T1 (&a0)[T2]) {
  return std::to_array<T1, T2>(a0);
}
