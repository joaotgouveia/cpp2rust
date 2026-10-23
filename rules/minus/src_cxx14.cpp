#include <functional>
#include <string>
#include <utility>

using t2 = std::minus<void>;

using t3 = std::minus<>::is_transparent;

std::minus<void> f2() { return std::minus<void>(); }

std::minus<void> f3(const std::minus<void> &a0) { return std::minus<void>(a0); }

std::minus<void> f4(std::minus<void> &&a0) {
  return std::minus<void>(std::move(a0));
}

std::minus<void> &f5(std::minus<void> &a0, const std::minus<void> &a1) {
  return a0.operator=(a1);
}

std::minus<void> &f6(std::minus<void> &a0, std::minus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::minus<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
