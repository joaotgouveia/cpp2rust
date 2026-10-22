#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

using t4 = std::plus<>;

std::plus<> f8() { return std::plus<>(); }

std::plus<> f9(const std::plus<void> &a0) { return std::plus<>(a0); }

std::plus<> f10(std::plus<void> &&a0) { return std::plus<>(std::move(a0)); }

std::plus<void> &f11(std::plus<> &a0, const std::plus<void> &a1) {
  return a0.operator=(a1);
}

std::plus<void> &f12(std::plus<> &a0, std::plus<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f13(const std::plus<> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
