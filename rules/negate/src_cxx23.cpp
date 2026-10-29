#include <barrier>
#include <utility>

using t1 = std::negate<void>;

using t2 = std::negate<>::is_transparent;

std::negate<void> f1() { return std::negate<void>(); }

std::negate<void> f2(const std::negate<void> &a0) {
  return std::negate<void>(a0);
}

std::negate<void> f3(std::negate<void> &&a0) {
  return std::negate<void>(std::move(a0));
}

std::negate<void> &f4(std::negate<void> &a0, const std::negate<void> &a1) {
  return a0.operator=(a1);
}

std::negate<void> &f5(std::negate<void> &a0, std::negate<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> auto f6(const std::negate<void> &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}

template <typename T1> using t3 = std::negate<T1>;

template <typename T1> T1 f7(const std::negate<T1> &a0, const T1 &a1) {
  return a0.operator()(a1);
}
