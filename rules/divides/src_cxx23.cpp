#include <barrier>
#include <utility>

using t1 = std::divides<void>;

using t2 = std::divides<>::is_transparent;

std::divides<void> f1() { return std::divides<void>(); }

std::divides<void> f2(const std::divides<void> &a0) {
  return std::divides<void>(a0);
}

std::divides<void> f3(std::divides<void> &&a0) {
  return std::divides<void>(std::move(a0));
}

std::divides<void> &f4(std::divides<void> &a0, const std::divides<void> &a1) {
  return a0.operator=(a1);
}

std::divides<void> &f5(std::divides<void> &a0, std::divides<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::divides<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::divides<T1>;

template <typename T1>
T1 f7(const std::divides<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
