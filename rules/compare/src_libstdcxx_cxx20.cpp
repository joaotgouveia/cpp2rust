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

template <typename T1>
std::partial_ordering f17(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1>
std::partial_ordering f18(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f19(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f20(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f21(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f22(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f23(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f24(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f25(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f26(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

bool f27(std::partial_ordering a0, std::partial_ordering a1) {
  return std::move(a0) == std::move(a1);
}

template <typename T1> bool f28(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

const std::partial_ordering &f29() { return std::partial_ordering::less; }

const std::partial_ordering &f30() { return std::partial_ordering::equivalent; }

const std::partial_ordering &f31() { return std::partial_ordering::greater; }

const std::partial_ordering &f32() { return std::partial_ordering::unordered; }

using t6 = std::strong_ordering;

std::partial_ordering f33(const std::strong_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f34(const std::strong_ordering &a0) {
  return static_cast<std::weak_ordering>(a0);
}

std::strong_ordering f35(const std::strong_ordering &a0) {
  return std::strong_ordering(a0);
}

std::strong_ordering f36(std::strong_ordering &&a0) {
  return std::strong_ordering(std::move(a0));
}

std::strong_ordering &f37(std::strong_ordering &a0,
                          const std::strong_ordering &a1) {
  return a0.operator=(a1);
}

std::strong_ordering &f38(std::strong_ordering &a0, std::strong_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::strong_ordering f39(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1>
std::strong_ordering f40(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f41(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f42(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f43(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f44(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f45(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f46(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f47(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f48(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

bool f49(std::strong_ordering a0, std::strong_ordering a1) {
  return std::move(a0) == std::move(a1);
}

template <typename T1> bool f50(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

const std::strong_ordering &f51() { return std::strong_ordering::less; }

const std::strong_ordering &f52() { return std::strong_ordering::equal; }

const std::strong_ordering &f53() { return std::strong_ordering::equivalent; }

const std::strong_ordering &f54() { return std::strong_ordering::greater; }

using t7 = std::weak_ordering;

std::partial_ordering f55(const std::weak_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f56(const std::weak_ordering &a0) {
  return std::weak_ordering(a0);
}

std::weak_ordering f57(std::weak_ordering &&a0) {
  return std::weak_ordering(std::move(a0));
}

std::weak_ordering &f58(std::weak_ordering &a0, const std::weak_ordering &a1) {
  return a0.operator=(a1);
}

std::weak_ordering &f59(std::weak_ordering &a0, std::weak_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::weak_ordering f60(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1> std::weak_ordering f61(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f62(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f63(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f64(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f65(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f66(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f67(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f68(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f69(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

bool f70(std::weak_ordering a0, std::weak_ordering a1) {
  return std::move(a0) == std::move(a1);
}

template <typename T1> bool f71(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

const std::weak_ordering &f72() { return std::weak_ordering::less; }

const std::weak_ordering &f73() { return std::weak_ordering::equivalent; }

const std::weak_ordering &f74() { return std::weak_ordering::greater; }
