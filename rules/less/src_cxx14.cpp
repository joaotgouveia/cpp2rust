#include <functional>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t2 = std::less<void>;

using t3 = std::less<>::is_transparent;

std::less<void> f2() { return std::less<void>(); }

std::less<void> f3(const std::less<void> &a0) { return std::less<void>(a0); }

std::less<void> f4(std::less<void> &&a0) {
  return std::less<void>(std::move(a0));
}

std::less<void> &f5(std::less<void> &a0, const std::less<void> &a1) {
  return a0.operator=(a1);
}

std::less<void> &f6(std::less<void> &a0, std::less<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::less<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
