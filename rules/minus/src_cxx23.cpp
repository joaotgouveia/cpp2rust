#include <barrier>
#include <utility>

using t1 = std::minus<void>;

using t2 = std::minus<>::is_transparent;

std::minus<void> f1() { return std::minus<void>(); }

std::minus<void> f2(const std::minus<void> &a0) { return std::minus<void>(a0); }

std::minus<void> f3(std::minus<void> &&a0) {
  return std::minus<void>(std::move(a0));
}

std::minus<void> &f4(std::minus<void> &a0, const std::minus<void> &a1) {
  return a0.operator=(a1);
}

std::minus<void> &f5(std::minus<void> &a0, std::minus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::minus<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::minus<T1>;

template <typename T1>
T1 f7(const std::minus<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
