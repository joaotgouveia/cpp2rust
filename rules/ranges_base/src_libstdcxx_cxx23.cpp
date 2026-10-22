#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

using t12 = std::from_range_t;

std::from_range_t f12() { return std::from_range_t(); }

std::from_range_t f13(const std::from_range_t &a0) {
  return std::from_range_t(a0);
}

std::from_range_t f14(std::from_range_t &&a0) {
  return std::from_range_t(std::move(a0));
}

std::from_range_t &f15(std::from_range_t &a0, const std::from_range_t &a1) {
  return a0.operator=(a1);
}

std::from_range_t &f16(std::from_range_t &a0, std::from_range_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t13 = std::ranges::const_iterator_t<T1>;

template <typename T1> using t14 = std::ranges::const_sentinel_t<T1>;

template <typename T1> using t15 = std::ranges::range_const_reference_t<T1>;
