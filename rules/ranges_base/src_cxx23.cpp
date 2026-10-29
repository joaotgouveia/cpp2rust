#include <algorithm>
#include <utility>

const std::from_range_t &f1() { return std::from_range; }

using t1 = std::from_range_t;

std::from_range_t f2() { return std::from_range_t(); }

std::from_range_t f3(const std::from_range_t &a0) {
  return std::from_range_t(a0);
}

std::from_range_t f4(std::from_range_t &&a0) {
  return std::from_range_t(std::move(a0));
}

std::from_range_t &f5(std::from_range_t &a0, const std::from_range_t &a1) {
  return a0.operator=(a1);
}

std::from_range_t &f6(std::from_range_t &a0, std::from_range_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t2 = std::ranges::borrowed_iterator_t<T1>;

template <typename T1> using t3 = std::ranges::const_iterator_t<T1>;

template <typename T1> using t4 = std::ranges::const_sentinel_t<T1>;

using t5 = std::ranges::dangling;

std::ranges::dangling f7() { return std::ranges::dangling(); }

std::ranges::dangling f8(const std::ranges::dangling &a0) {
  return std::ranges::dangling(a0);
}

std::ranges::dangling f9(std::ranges::dangling &&a0) {
  return std::ranges::dangling(std::move(a0));
}

std::ranges::dangling &f10(std::ranges::dangling &a0,
                           const std::ranges::dangling &a1) {
  return a0.operator=(a1);
}

std::ranges::dangling &f11(std::ranges::dangling &a0,
                           std::ranges::dangling &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> std::ranges::dangling f12(T1 &&...a0) {
  return std::ranges::dangling(std::forward<T1>(a0)...);
}

template <typename T1> bool f13() {
  return std::ranges::disable_sized_range<T1>;
}

template <typename T1> bool f14() {
  return std::ranges::enable_borrowed_range<T1>;
}

template <typename T1> bool f15() { return std::ranges::enable_view<T1>; }

template <typename T1> using t6 = std::ranges::iterator_t<T1>;

template <typename T1> using t7 = std::ranges::range_common_reference_t<T1>;

template <typename T1> using t8 = std::ranges::range_const_reference_t<T1>;

template <typename T1> using t9 = std::ranges::range_difference_t<T1>;

template <typename T1> using t10 = std::ranges::range_reference_t<T1>;

template <typename T1> using t11 = std::ranges::range_rvalue_reference_t<T1>;

template <typename T1> using t12 = std::ranges::range_size_t<T1>;

template <typename T1> using t13 = std::ranges::range_value_t<T1>;

template <typename T1> using t14 = std::ranges::sentinel_t<T1>;

using t15 = std::ranges::view_base;

std::ranges::view_base f16() { return std::ranges::view_base(); }

std::ranges::view_base f17(const std::ranges::view_base &a0) {
  return std::ranges::view_base(a0);
}

std::ranges::view_base f18(std::ranges::view_base &&a0) {
  return std::ranges::view_base(std::move(a0));
}

std::ranges::view_base &f19(std::ranges::view_base &a0,
                            const std::ranges::view_base &a1) {
  return a0.operator=(a1);
}

std::ranges::view_base &f20(std::ranges::view_base &a0,
                            std::ranges::view_base &&a1) {
  return a0.operator=(std::move(a1));
}
