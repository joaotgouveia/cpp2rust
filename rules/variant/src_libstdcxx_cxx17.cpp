#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <type_traits>
#include <utility>
#include <variant>

using t3 = std::bad_variant_access;

std::bad_variant_access f38() { return std::bad_variant_access(); }

const char *f39(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f40(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f41(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f42(std::bad_variant_access &a0,
                             const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f43(std::bad_variant_access &a0,
                             std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f44(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename T1, typename... T2> auto f45(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
