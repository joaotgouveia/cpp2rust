#include <algorithm>
#include <type_traits>
#include <utility>
#include <variant>

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

template <typename... T1, typename T2, typename... T3,
          typename T4 = std::enable_if_t<
              std::variant<T1...>::template __exactly_once<T2> &&
              std::is_constructible_v<T2, T3...>>>
std::variant<T1...> f35(std::in_place_type_t<T2> a0, T3 &&...a1) {
  return std::variant<T1...>(a0, std::forward<T3>(a1)...);
}

template <typename... T1, typename T2, typename T3, typename... T4,
          typename T5 = std::enable_if_t<
              std::variant<T1...>::template __exactly_once<T2> &&
              std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>>>
std::variant<T1...> f36(std::in_place_type_t<T2> a0,
                        std::initializer_list<T3> a1, T4 &&...a2) {
  return std::variant<T1...>(a0, a1, std::forward<T4>(a2)...);
}

template <typename T1, typename... T2> auto f37(T1 &&a0, T2 &&...a1) {
  return std::visit<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
