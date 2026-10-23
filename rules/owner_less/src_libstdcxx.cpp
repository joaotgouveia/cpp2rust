#include <condition_variable>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t3 = std::owner_less<void>;

std::owner_less<void> f7() { return std::owner_less<void>(); }

std::owner_less<void> f8(const std::owner_less<void> &a0) {
  return std::owner_less<void>(a0);
}

std::owner_less<void> f9(std::owner_less<void> &&a0) {
  return std::owner_less<void>(std::move(a0));
}

std::owner_less<void> &f10(std::owner_less<void> &a0,
                           const std::owner_less<void> &a1) {
  return a0.operator=(a1);
}

std::owner_less<void> &f11(std::owner_less<void> &a0,
                           std::owner_less<void> &&a1) {
  return a0.operator=(std::move(a1));
}
