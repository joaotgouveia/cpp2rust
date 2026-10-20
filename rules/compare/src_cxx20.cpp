#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <utility>

template <typename... T1> using t1 = std::common_comparison_category_t<T1...>;

using t2 = std::compare_three_way;

using t3 = std::compare_three_way::is_transparent;

std::compare_three_way f1() { return std::compare_three_way(); }

std::compare_three_way f2(const std::compare_three_way &a0) {
  return std::compare_three_way(a0);
}

std::compare_three_way f3(std::compare_three_way &&a0) {
  return std::compare_three_way(std::move(a0));
}

std::compare_three_way &f4(std::compare_three_way &a0,
                           const std::compare_three_way &a1) {
  return a0.operator=(a1);
}

std::compare_three_way &f5(std::compare_three_way &a0,
                           std::compare_three_way &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::compare_three_way &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1, typename T2>
using t4 = std::compare_three_way_result_t<T1, T2>;

bool f7(std::partial_ordering a0) { return std::is_eq(std::move(a0)); }

bool f8(std::partial_ordering a0) { return std::is_gt(std::move(a0)); }

bool f9(std::partial_ordering a0) { return std::is_gteq(std::move(a0)); }

bool f10(std::partial_ordering a0) { return std::is_lt(std::move(a0)); }

bool f11(std::partial_ordering a0) { return std::is_lteq(std::move(a0)); }

bool f12(std::partial_ordering a0) { return std::is_neq(std::move(a0)); }

using t5 = std::partial_ordering;

std::partial_ordering f13(const std::partial_ordering &a0) {
  return std::partial_ordering(a0);
}

std::partial_ordering f14(std::partial_ordering &&a0) {
  return std::partial_ordering(std::move(a0));
}

std::partial_ordering &f15(std::partial_ordering &a0,
                           const std::partial_ordering &a1) {
  return a0.operator=(a1);
}

std::partial_ordering &f16(std::partial_ordering &a0,
                           std::partial_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f17(std::partial_ordering a0, std::partial_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::partial_ordering &f18() { return std::partial_ordering::less; }

const std::partial_ordering &f19() { return std::partial_ordering::equivalent; }

const std::partial_ordering &f20() { return std::partial_ordering::greater; }

const std::partial_ordering &f21() { return std::partial_ordering::unordered; }

using t6 = std::strong_ordering;

std::partial_ordering f22(const std::strong_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f23(const std::strong_ordering &a0) {
  return static_cast<std::weak_ordering>(a0);
}

std::strong_ordering f24(const std::strong_ordering &a0) {
  return std::strong_ordering(a0);
}

std::strong_ordering f25(std::strong_ordering &&a0) {
  return std::strong_ordering(std::move(a0));
}

std::strong_ordering &f26(std::strong_ordering &a0,
                          const std::strong_ordering &a1) {
  return a0.operator=(a1);
}

std::strong_ordering &f27(std::strong_ordering &a0, std::strong_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f28(std::strong_ordering a0, std::strong_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::strong_ordering &f29() { return std::strong_ordering::less; }

const std::strong_ordering &f30() { return std::strong_ordering::equal; }

const std::strong_ordering &f31() { return std::strong_ordering::equivalent; }

const std::strong_ordering &f32() { return std::strong_ordering::greater; }

using t7 = std::weak_ordering;

std::partial_ordering f33(const std::weak_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f34(const std::weak_ordering &a0) {
  return std::weak_ordering(a0);
}

std::weak_ordering f35(std::weak_ordering &&a0) {
  return std::weak_ordering(std::move(a0));
}

std::weak_ordering &f36(std::weak_ordering &a0, const std::weak_ordering &a1) {
  return a0.operator=(a1);
}

std::weak_ordering &f37(std::weak_ordering &a0, std::weak_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f38(std::weak_ordering a0, std::weak_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::weak_ordering &f39() { return std::weak_ordering::less; }

const std::weak_ordering &f40() { return std::weak_ordering::equivalent; }

const std::weak_ordering &f41() { return std::weak_ordering::greater; }
