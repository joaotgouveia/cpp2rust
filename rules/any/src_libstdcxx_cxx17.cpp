#include <algorithm>
#include <any>
#include <type_traits>
#include <utility>

template <typename T1>
std::enable_if_t<
    std::is_copy_constructible<std::enable_if_t<
        !std::is_same_v<std::decay_t<T1>, std::any>, std::decay_t<T1>>>::value,
    std::any &>
f14(std::any &a0, T1 &&a1) {
  return a0.template operator= <T1>(std::forward<T1>(a1));
}

template <typename T1> T1 *f15(std::any *a0) { return std::any_cast<T1>(a0); }

template <typename T1> const T1 *f16(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

using t2 = std::bad_any_cast;

const char *f17(const std::bad_any_cast &a0) { return a0.what(); }

std::bad_any_cast f18() { return std::bad_any_cast(); }

std::bad_any_cast f19(const std::bad_any_cast &a0) {
  return std::bad_any_cast(a0);
}

std::bad_any_cast f20(std::bad_any_cast &&a0) {
  return std::bad_any_cast(std::move(a0));
}

std::bad_any_cast &f21(std::bad_any_cast &a0, const std::bad_any_cast &a1) {
  return a0.operator=(a1);
}

std::bad_any_cast &f22(std::bad_any_cast &a0, std::bad_any_cast &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
std::enable_if_t<
    std::is_constructible_v<std::any, std::in_place_type_t<T1>, T2...>,
    std::any>
f23(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<std::is_constructible_v<std::any, std::in_place_type_t<T1>,
                                         std::initializer_list<T2> &, T3...>,
                 std::any>
f24(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}
