#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

using t1 = std::identity;

using t2 = std::identity::is_transparent;

std::identity f1() { return std::identity(); }

std::identity f2(const std::identity &a0) { return std::identity(a0); }

std::identity f3(std::identity &&a0) { return std::identity(std::move(a0)); }

std::identity &f4(std::identity &a0, const std::identity &a1) {
  return a0.operator=(a1);
}

std::identity &f5(std::identity &a0, std::identity &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 &&f6(const std::identity &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}

using t3 = std::ranges::equal_to;

using t4 = std::ranges::equal_to::is_transparent;

std::ranges::equal_to f7() { return std::ranges::equal_to(); }

std::ranges::equal_to f8(const std::ranges::equal_to &a0) {
  return std::ranges::equal_to(a0);
}

std::ranges::equal_to f9(std::ranges::equal_to &&a0) {
  return std::ranges::equal_to(std::move(a0));
}

std::ranges::equal_to &f10(std::ranges::equal_to &a0,
                           const std::ranges::equal_to &a1) {
  return a0.operator=(a1);
}

std::ranges::equal_to &f11(std::ranges::equal_to &a0,
                           std::ranges::equal_to &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f12(const std::ranges::equal_to &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t5 = std::ranges::greater;

using t6 = std::ranges::greater::is_transparent;

std::ranges::greater f13() { return std::ranges::greater(); }

std::ranges::greater f14(const std::ranges::greater &a0) {
  return std::ranges::greater(a0);
}

std::ranges::greater f15(std::ranges::greater &&a0) {
  return std::ranges::greater(std::move(a0));
}

std::ranges::greater &f16(std::ranges::greater &a0,
                          const std::ranges::greater &a1) {
  return a0.operator=(a1);
}

std::ranges::greater &f17(std::ranges::greater &a0, std::ranges::greater &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f18(const std::ranges::greater &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t7 = std::ranges::greater_equal;

using t8 = std::ranges::greater_equal::is_transparent;

std::ranges::greater_equal f19() { return std::ranges::greater_equal(); }

std::ranges::greater_equal f20(const std::ranges::greater_equal &a0) {
  return std::ranges::greater_equal(a0);
}

std::ranges::greater_equal f21(std::ranges::greater_equal &&a0) {
  return std::ranges::greater_equal(std::move(a0));
}

std::ranges::greater_equal &f22(std::ranges::greater_equal &a0,
                                const std::ranges::greater_equal &a1) {
  return a0.operator=(a1);
}

std::ranges::greater_equal &f23(std::ranges::greater_equal &a0,
                                std::ranges::greater_equal &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f24(const std::ranges::greater_equal &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t9 = std::ranges::less;

using t10 = std::ranges::less::is_transparent;

std::ranges::less f25() { return std::ranges::less(); }

std::ranges::less f26(const std::ranges::less &a0) {
  return std::ranges::less(a0);
}

std::ranges::less f27(std::ranges::less &&a0) {
  return std::ranges::less(std::move(a0));
}

std::ranges::less &f28(std::ranges::less &a0, const std::ranges::less &a1) {
  return a0.operator=(a1);
}

std::ranges::less &f29(std::ranges::less &a0, std::ranges::less &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f30(const std::ranges::less &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t11 = std::ranges::less_equal;

using t12 = std::ranges::less_equal::is_transparent;

std::ranges::less_equal f31() { return std::ranges::less_equal(); }

std::ranges::less_equal f32(const std::ranges::less_equal &a0) {
  return std::ranges::less_equal(a0);
}

std::ranges::less_equal f33(std::ranges::less_equal &&a0) {
  return std::ranges::less_equal(std::move(a0));
}

std::ranges::less_equal &f34(std::ranges::less_equal &a0,
                             const std::ranges::less_equal &a1) {
  return a0.operator=(a1);
}

std::ranges::less_equal &f35(std::ranges::less_equal &a0,
                             std::ranges::less_equal &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f36(const std::ranges::less_equal &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t13 = std::ranges::not_equal_to;

using t14 = std::ranges::not_equal_to::is_transparent;

std::ranges::not_equal_to f37() { return std::ranges::not_equal_to(); }

std::ranges::not_equal_to f38(const std::ranges::not_equal_to &a0) {
  return std::ranges::not_equal_to(a0);
}

std::ranges::not_equal_to f39(std::ranges::not_equal_to &&a0) {
  return std::ranges::not_equal_to(std::move(a0));
}

std::ranges::not_equal_to &f40(std::ranges::not_equal_to &a0,
                               const std::ranges::not_equal_to &a1) {
  return a0.operator=(a1);
}

std::ranges::not_equal_to &f41(std::ranges::not_equal_to &a0,
                               std::ranges::not_equal_to &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f42(const std::ranges::not_equal_to &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
