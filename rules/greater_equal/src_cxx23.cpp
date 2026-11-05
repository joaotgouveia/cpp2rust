#include <barrier>
#include <utility>

using t1 = std::greater_equal<void>;

using t2 = std::greater_equal<>::is_transparent;

std::greater_equal<void> f1() { return std::greater_equal<void>(); }

std::greater_equal<void> f2(const std::greater_equal<void> &a0) {
  return std::greater_equal<void>(a0);
}

std::greater_equal<void> f3(std::greater_equal<void> &&a0) {
  return std::greater_equal<void>(std::move(a0));
}

std::greater_equal<void> &f4(std::greater_equal<void> &a0,
                             const std::greater_equal<void> &a1) {
  return a0.operator=(a1);
}

std::greater_equal<void> &f5(std::greater_equal<void> &a0,
                             std::greater_equal<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::greater_equal<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1, typename T2>
bool f7(const std::greater_equal<void> &a0, T1 *a1, T2 *a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1> using t3 = std::greater_equal<T1>;

template <typename T1>
bool f8(const std::greater_equal<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
