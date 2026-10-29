#include <barrier>
#include <utility>

using t1 = std::logical_not<void>;

using t2 = std::logical_not<>::is_transparent;

std::logical_not<void> f1() { return std::logical_not<void>(); }

std::logical_not<void> f2(const std::logical_not<void> &a0) {
  return std::logical_not<void>(a0);
}

std::logical_not<void> f3(std::logical_not<void> &&a0) {
  return std::logical_not<void>(std::move(a0));
}

std::logical_not<void> &f4(std::logical_not<void> &a0,
                           const std::logical_not<void> &a1) {
  return a0.operator=(a1);
}

std::logical_not<void> &f5(std::logical_not<void> &a0,
                           std::logical_not<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> auto f6(const std::logical_not<void> &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}

template <typename T1> using t3 = std::logical_not<T1>;

template <typename T1> bool f7(const std::logical_not<T1> &a0, const T1 &a1) {
  return a0.operator()(a1);
}
