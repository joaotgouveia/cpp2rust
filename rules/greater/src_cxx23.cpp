#include <barrier>
#include <utility>

using t1 = std::greater<void>;

using t2 = std::greater<>::is_transparent;

std::greater<void> f1() { return std::greater<void>(); }

std::greater<void> f2(const std::greater<void> &a0) {
  return std::greater<void>(a0);
}

std::greater<void> f3(std::greater<void> &&a0) {
  return std::greater<void>(std::move(a0));
}

std::greater<void> &f4(std::greater<void> &a0, const std::greater<void> &a1) {
  return a0.operator=(a1);
}

std::greater<void> &f5(std::greater<void> &a0, std::greater<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::greater<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1, typename T2>
bool f7(const std::greater<void> &a0, T1 *a1, T2 *a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1> using t3 = std::greater<T1>;

template <typename T1>
bool f8(const std::greater<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
