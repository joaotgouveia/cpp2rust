#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::ranges::borrowed_iterator_t<T1>;

using t2 = std::ranges::dangling;

std::ranges::dangling f1() { return std::ranges::dangling(); }

std::ranges::dangling f2(const std::ranges::dangling &a0) {
  return std::ranges::dangling(a0);
}

std::ranges::dangling f3(std::ranges::dangling &&a0) {
  return std::ranges::dangling(std::move(a0));
}

std::ranges::dangling &f4(std::ranges::dangling &a0,
                          const std::ranges::dangling &a1) {
  return a0.operator=(a1);
}

std::ranges::dangling &f5(std::ranges::dangling &a0,
                          std::ranges::dangling &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> std::ranges::dangling f6(T1 &&...a0) {
  return std::ranges::dangling(std::forward<T1>(a0)...);
}

template <typename T1> using t3 = std::ranges::iterator_t<T1>;

template <typename T1> using t4 = std::ranges::range_common_reference_t<T1>;

template <typename T1> using t5 = std::ranges::range_difference_t<T1>;

template <typename T1> using t6 = std::ranges::range_reference_t<T1>;

template <typename T1> using t7 = std::ranges::range_rvalue_reference_t<T1>;

template <typename T1> using t8 = std::ranges::range_size_t<T1>;

template <typename T1> using t9 = std::ranges::range_value_t<T1>;

template <typename T1> using t10 = std::ranges::sentinel_t<T1>;

using t11 = std::ranges::view_base;

std::ranges::view_base f7() { return std::ranges::view_base(); }

std::ranges::view_base f8(const std::ranges::view_base &a0) {
  return std::ranges::view_base(a0);
}

std::ranges::view_base f9(std::ranges::view_base &&a0) {
  return std::ranges::view_base(std::move(a0));
}

std::ranges::view_base &f10(std::ranges::view_base &a0,
                            const std::ranges::view_base &a1) {
  return a0.operator=(a1);
}

std::ranges::view_base &f11(std::ranges::view_base &a0,
                            std::ranges::view_base &&a1) {
  return a0.operator=(std::move(a1));
}
