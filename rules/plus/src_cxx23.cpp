#include <barrier>
#include <utility>

using t1 = std::plus<void>;

using t2 = std::plus<>::is_transparent;

std::plus<void> f1() { return std::plus<void>(); }

std::plus<void> f2(const std::plus<void> &a0) { return std::plus<void>(a0); }

std::plus<void> f3(std::plus<void> &&a0) {
  return std::plus<void>(std::move(a0));
}

std::plus<void> &f4(std::plus<void> &a0, const std::plus<void> &a1) {
  return a0.operator=(a1);
}

std::plus<void> &f5(std::plus<void> &a0, std::plus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::plus<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::plus<T1>;

template <typename T1>
T1 f7(const std::plus<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
