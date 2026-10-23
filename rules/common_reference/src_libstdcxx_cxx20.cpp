#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t3 = std::common_reference<>;

std::common_reference<> f1() { return std::common_reference<>(); }

std::common_reference<> f2(const std::common_reference<> &a0) {
  return std::common_reference<>(a0);
}

std::common_reference<> f3(std::common_reference<> &&a0) {
  return std::common_reference<>(std::move(a0));
}

std::common_reference<> &f4(std::common_reference<> &a0,
                            const std::common_reference<> &a1) {
  return a0.operator=(a1);
}

std::common_reference<> &f5(std::common_reference<> &a0,
                            std::common_reference<> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> using t4 = std::common_reference<T1, T2>;

template <typename T1, typename T2, typename... T3>
using t5 = std::common_reference<T1, T2, T3...>;
