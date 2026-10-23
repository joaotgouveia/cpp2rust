#include <functional>
#include <utility>

using t1 = std::ranges::equal_to;

using t2 = std::ranges::equal_to::is_transparent;

std::ranges::equal_to f1() { return std::ranges::equal_to(); }

std::ranges::equal_to f2(const std::ranges::equal_to &a0) {
  return std::ranges::equal_to(a0);
}

std::ranges::equal_to f3(std::ranges::equal_to &&a0) {
  return std::ranges::equal_to(std::move(a0));
}

std::ranges::equal_to &f4(std::ranges::equal_to &a0,
                          const std::ranges::equal_to &a1) {
  return a0.operator=(a1);
}

std::ranges::equal_to &f5(std::ranges::equal_to &a0,
                          std::ranges::equal_to &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f6(const std::ranges::equal_to &a0, T1 &&a1, T2 &&a2)
  requires(std::equality_comparable_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t3 = std::ranges::greater;

using t4 = std::ranges::greater::is_transparent;

std::ranges::greater f7() { return std::ranges::greater(); }

std::ranges::greater f8(const std::ranges::greater &a0) {
  return std::ranges::greater(a0);
}

std::ranges::greater f9(std::ranges::greater &&a0) {
  return std::ranges::greater(std::move(a0));
}

std::ranges::greater &f10(std::ranges::greater &a0,
                          const std::ranges::greater &a1) {
  return a0.operator=(a1);
}

std::ranges::greater &f11(std::ranges::greater &a0, std::ranges::greater &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f12(const std::ranges::greater &a0, T1 &&a1, T2 &&a2)
  requires(std::totally_ordered_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t5 = std::ranges::greater_equal;

using t6 = std::ranges::greater_equal::is_transparent;

std::ranges::greater_equal f13() { return std::ranges::greater_equal(); }

std::ranges::greater_equal f14(const std::ranges::greater_equal &a0) {
  return std::ranges::greater_equal(a0);
}

std::ranges::greater_equal f15(std::ranges::greater_equal &&a0) {
  return std::ranges::greater_equal(std::move(a0));
}

std::ranges::greater_equal &f16(std::ranges::greater_equal &a0,
                                const std::ranges::greater_equal &a1) {
  return a0.operator=(a1);
}

std::ranges::greater_equal &f17(std::ranges::greater_equal &a0,
                                std::ranges::greater_equal &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f18(const std::ranges::greater_equal &a0, T1 &&a1, T2 &&a2)
  requires(std::totally_ordered_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t7 = std::ranges::less;

using t8 = std::ranges::less::is_transparent;

std::ranges::less f19() { return std::ranges::less(); }

std::ranges::less f20(const std::ranges::less &a0) {
  return std::ranges::less(a0);
}

std::ranges::less f21(std::ranges::less &&a0) {
  return std::ranges::less(std::move(a0));
}

std::ranges::less &f22(std::ranges::less &a0, const std::ranges::less &a1) {
  return a0.operator=(a1);
}

std::ranges::less &f23(std::ranges::less &a0, std::ranges::less &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f24(const std::ranges::less &a0, T1 &&a1, T2 &&a2)
  requires(std::totally_ordered_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t9 = std::ranges::less_equal;

using t10 = std::ranges::less_equal::is_transparent;

std::ranges::less_equal f25() { return std::ranges::less_equal(); }

std::ranges::less_equal f26(const std::ranges::less_equal &a0) {
  return std::ranges::less_equal(a0);
}

std::ranges::less_equal f27(std::ranges::less_equal &&a0) {
  return std::ranges::less_equal(std::move(a0));
}

std::ranges::less_equal &f28(std::ranges::less_equal &a0,
                             const std::ranges::less_equal &a1) {
  return a0.operator=(a1);
}

std::ranges::less_equal &f29(std::ranges::less_equal &a0,
                             std::ranges::less_equal &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f30(const std::ranges::less_equal &a0, T1 &&a1, T2 &&a2)
  requires(std::totally_ordered_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

using t11 = std::ranges::not_equal_to;

using t12 = std::ranges::not_equal_to::is_transparent;

std::ranges::not_equal_to f31() { return std::ranges::not_equal_to(); }

std::ranges::not_equal_to f32(const std::ranges::not_equal_to &a0) {
  return std::ranges::not_equal_to(a0);
}

std::ranges::not_equal_to f33(std::ranges::not_equal_to &&a0) {
  return std::ranges::not_equal_to(std::move(a0));
}

std::ranges::not_equal_to &f34(std::ranges::not_equal_to &a0,
                               const std::ranges::not_equal_to &a1) {
  return a0.operator=(a1);
}

std::ranges::not_equal_to &f35(std::ranges::not_equal_to &a0,
                               std::ranges::not_equal_to &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f36(const std::ranges::not_equal_to &a0, T1 &&a1, T2 &&a2)
  requires(std::equality_comparable_with<T1, T2>)
{
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
