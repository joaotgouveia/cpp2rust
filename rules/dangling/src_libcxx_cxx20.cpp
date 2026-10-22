#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::borrowed_iterator_t<T1>;

using t2 = std::ranges::dangling;

std::ranges::dangling f1() { return std::ranges::dangling(); }

std::ranges::dangling f2(const std::ranges::dangling &a0) {
  return std::ranges::dangling(a0);
}

std::ranges::dangling f3(std::ranges::dangling &&a0) {
  return std::ranges::dangling(std::move(a0));
}

std::ranges::dangling &f4(std::ranges::dangling &a0,
                          const std::ranges::dangling &a1) {
  return a0.operator=(a1);
}

std::ranges::dangling &f5(std::ranges::dangling &a0,
                          std::ranges::dangling &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1> std::ranges::dangling f6(T1 &&...a0) {
  return std::ranges::dangling(std::forward<T1>(a0)...);
}
