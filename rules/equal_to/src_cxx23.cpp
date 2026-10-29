#include <barrier>
#include <utility>

using t1 = std::equal_to<void>;

using t2 = std::equal_to<>::is_transparent;

std::equal_to<void> f1() { return std::equal_to<void>(); }

std::equal_to<void> f2(const std::equal_to<void> &a0) {
  return std::equal_to<void>(a0);
}

std::equal_to<void> f3(std::equal_to<void> &&a0) {
  return std::equal_to<void>(std::move(a0));
}

std::equal_to<void> &f4(std::equal_to<void> &a0,
                        const std::equal_to<void> &a1) {
  return a0.operator=(a1);
}

std::equal_to<void> &f5(std::equal_to<void> &a0, std::equal_to<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::equal_to<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::equal_to<T1>;

template <typename T1>
bool f7(const std::equal_to<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
