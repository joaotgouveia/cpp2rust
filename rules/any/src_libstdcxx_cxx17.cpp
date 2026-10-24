#include <algorithm>
#include <any>
#include <utility>

template <
    typename T1, typename... T2, typename T3 = std::decay_t<T1>,
    typename T4 = std::__conditional_t<
        std::integral_constant<
            bool,
            std::is_nothrow_move_constructible<std::decay_t<T1>>::value &&
                (sizeof(std::decay_t<T1>) <= sizeof(std::any::_Storage)) &&
                (alignof(std::decay_t<T1>) <=
                 alignof(std::any::_Storage))>::value,
        std::any::_Manager_internal<std::decay_t<T1>>,
        std::any::_Manager_external<std::decay_t<T1>>>,
    typename std::enable_if<
        std::__and_<std::is_copy_constructible<std::decay_t<T1>>,
                    std::is_constructible<std::decay_t<T1>, T2 &&...>>::value,
        bool>::type T5 = false>
std::any f16(std::in_place_type_t<T1> a0, T2 &&...a1) {
  return std::any(std::move(a0), std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3, typename T4 = std::decay_t<T1>,
    typename T5 = std::__conditional_t<
        std::integral_constant<
            bool,
            std::is_nothrow_move_constructible<std::decay_t<T1>>::value &&
                (sizeof(std::decay_t<T1>) <= sizeof(std::any::_Storage)) &&
                (alignof(std::decay_t<T1>) <=
                 alignof(std::any::_Storage))>::value,
        std::any::_Manager_internal<std::decay_t<T1>>,
        std::any::_Manager_external<std::decay_t<T1>>>,
    typename std::enable_if<
        std::__and_<
            std::is_copy_constructible<std::decay_t<T1>>,
            std::is_constructible<std::decay_t<T1>, std::initializer_list<T2> &,
                                  T3 &&...>>::value,
        bool>::type T6 = false>
std::any f17(std::in_place_type_t<T1> a0, std::initializer_list<T2> a1,
             T3 &&...a2) {
  return std::any(std::move(a0), std::move(a1), std::forward<T3>(a2)...);
}

template <typename T1, typename... T2>
typename std::enable_if<
    std::__and_<std::is_copy_constructible<std::decay_t<T1>>,
                std::is_constructible<std::decay_t<T1>, T2...>>::value,
    std::decay_t<T1> &>::type
f18(std::any &a0, T2 &&...a1) {
  return a0.template emplace<T1, T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::enable_if<
    std::__and_<
        std::is_copy_constructible<std::decay_t<T1>>,
        std::is_constructible<std::decay_t<T1>, std::initializer_list<T2> &,
                              T3 &&...>>::value,
    std::decay_t<T1> &>::type
f19(std::any &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T1, T2, T3...>(std::move(a1),
                                            std::forward<T3>(a2)...);
}

template <typename T1> T1 *f20(std::any *a0) { return std::any_cast<T1>(a0); }

template <typename T1> const T1 *f21(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

using t2 = std::bad_any_cast;

const char *f22(const std::bad_any_cast &a0) { return a0.what(); }

std::bad_any_cast f23() { return std::bad_any_cast(); }

std::bad_any_cast f24(const std::bad_any_cast &a0) {
  return std::bad_any_cast(a0);
}

std::bad_any_cast f25(std::bad_any_cast &&a0) {
  return std::bad_any_cast(std::move(a0));
}

std::bad_any_cast &f26(std::bad_any_cast &a0, const std::bad_any_cast &a1) {
  return a0.operator=(a1);
}

std::bad_any_cast &f27(std::bad_any_cast &a0, std::bad_any_cast &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
std::enable_if_t<
    std::is_constructible_v<std::any, std::in_place_type_t<T1>, T2...>,
    std::any>
f28(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<std::is_constructible_v<std::any, std::in_place_type_t<T1>,
                                         std::initializer_list<T2> &, T3...>,
                 std::any>
f29(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(std::move(a0), std::forward<T3>(a1)...);
}
