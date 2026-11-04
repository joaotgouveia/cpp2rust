#include <barrier>
#include <utility>

using t1 = std::multiplies<void>;

using t2 = std::multiplies<>::is_transparent;

std::multiplies<void> f1() { return std::multiplies<void>(); }

std::multiplies<void> f2(const std::multiplies<void> &a0) {
  return std::multiplies<void>(a0);
}

std::multiplies<void> f3(std::multiplies<void> &&a0) {
  return std::multiplies<void>(std::move(a0));
}

std::multiplies<void> &f4(std::multiplies<void> &a0,
                          const std::multiplies<void> &a1) {
  return a0.operator=(a1);
}

std::multiplies<void> &f5(std::multiplies<void> &a0,
                          std::multiplies<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::multiplies<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::multiplies<T1>;

template <typename T1>
T1 f7(const std::multiplies<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
