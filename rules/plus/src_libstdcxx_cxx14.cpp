#include <string>
#include <utility>

using t3 = std::plus<void>;

std::plus<void> f2() { return std::plus<void>(); }

std::plus<void> f3(const std::plus<void> &a0) { return std::plus<void>(a0); }

std::plus<void> f4(std::plus<void> &&a0) {
  return std::plus<void>(std::move(a0));
}

std::plus<void> &f5(std::plus<void> &a0, const std::plus<void> &a1) {
  return a0.operator=(a1);
}

std::plus<void> &f6(std::plus<void> &a0, std::plus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::plus<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
