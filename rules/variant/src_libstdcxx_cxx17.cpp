#include <algorithm>
#include <type_traits>
#include <utility>
#include <variant>

#include "../common/init.h"
#include "../common/traits.h"

using t3 = std::bad_variant_access;

std::bad_variant_access f27() { return std::bad_variant_access(); }

const char *f28(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f29(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f30(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f31(std::bad_variant_access &a0,
                             const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f32(std::bad_variant_access &a0,
                             std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
std::enable_if_t<(std::is_move_constructible_v<T1> && ...) &&
                 (std::is_swappable_v<T1> && ...)>
f33(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename... T1>
void f34(std::variant<T1...> &a0, std::variant<T1...> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename... T2> auto f35(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
