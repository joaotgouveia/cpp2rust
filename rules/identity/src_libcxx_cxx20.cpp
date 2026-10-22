#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

using t1 = std::identity;

using t2 = std::identity::is_transparent;

std::identity f1() { return std::identity(); }

std::identity f2(const std::identity &a0) { return std::identity(a0); }

std::identity f3(std::identity &&a0) { return std::identity(std::move(a0)); }

std::identity &f4(std::identity &a0, const std::identity &a1) {
  return a0.operator=(a1);
}

std::identity &f5(std::identity &a0, std::identity &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 &&f6(const std::identity &a0, T1 &&a1) {
  return a0.template operator()<T1>(std::forward<T1>(a1));
}
