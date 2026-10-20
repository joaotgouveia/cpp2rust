#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

using t2 = std::less_equal<void>;

using t3 = std::less_equal<>::is_transparent;

std::less_equal<void> f2() { return std::less_equal<void>(); }

std::less_equal<void> f3(const std::less_equal<void> &a0) {
  return std::less_equal<void>(a0);
}

std::less_equal<void> f4(std::less_equal<void> &&a0) {
  return std::less_equal<void>(std::move(a0));
}

std::less_equal<void> &f5(std::less_equal<void> &a0,
                          const std::less_equal<void> &a1) {
  return a0.operator=(a1);
}

std::less_equal<void> &f6(std::less_equal<void> &a0,
                          std::less_equal<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::less_equal<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1, typename T2>
bool f8(const std::less_equal<void> &a0, T1 *a1, T2 *a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}
