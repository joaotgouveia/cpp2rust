#include <algorithm>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <optional>
#include <system_error>
#include <typeindex>
#include <utility>
#include <variant>
#include <vector>

template <typename T1>
using t57 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type;

template <typename T1>
using t58 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::result_type;

template <typename T1>
using t59 = typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type::value_type;

template <typename T1>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f256() {
  return typename std::hash<std::__enable_hash_helper<
      std::optional<T1>, std::remove_const_t<T1>>>::argument_type();
}

template <typename T1>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f257(const typename std::hash<std::optional<T1>>::argument_type &a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::optional<T1>, std::remove_const_t<T1>>>::argument_type(a0);
}

template <typename T1>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f258(typename std::hash<std::optional<T1>>::argument_type &&a0) {
  return typename std::hash<
      std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>::
      argument_type(std::move(a0));
}

template <typename T1>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f259(std::nullopt_t a0) {
  return typename std::hash<
      std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>::
      argument_type(std::move(a0));
}

template <typename T1>
typename std::hash<std::optional<T1>>::argument_type &
f260(typename std::hash<std::__enable_hash_helper<
         std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
     std::nullopt_t a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::hash<std::optional<T1>>::argument_type &
f261(typename std::hash<std::__enable_hash_helper<
         std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
     const typename std::hash<std::optional<T1>>::argument_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::hash<std::optional<T1>>::argument_type &
f262(typename std::hash<std::__enable_hash_helper<
         std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
     typename std::hash<std::optional<T1>>::argument_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f263(typename std::hash<std::__enable_hash_helper<
              std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
          typename std::hash<std::optional<T1>>::argument_type &a1) {
  return a0.swap(a1);
}

template <typename T1>
bool f264(const typename std::hash<std::__enable_hash_helper<
              std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0) {
  return static_cast<bool>(a0);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::integral_constant<
            bool, std::integral_constant<
                      bool, std::is_same<T2, std::in_place_t>::value>::value &&
                      std::is_constructible<T1, T3...>::value>::value,
        int>
        T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f265(T2 a0, T3 &&...a1) {
  return typename std::hash<
      std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>::
      argument_type(a0, std::forward<T3>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::optional<T1>, std::remove_const_t<T1>>>::argument_type
f266(std::in_place_t a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return typename std::hash<
      std::__enable_hash_helper<std::optional<T1>, std::remove_const_t<T1>>>::
      argument_type(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T2>::type>::type,
                          std::optional<T1>>::value>::value &&
                std::integral_constant<
                    bool,
                    std::integral_constant<
                        bool, !std::is_same<typename std::remove_cv<
                                                typename std::remove_reference<
                                                    T2>::type>::type,
                                            T1>::value>::value ||
                        std::integral_constant<
                            bool, !std::is_scalar<T1>::value>::value>::value &&
                std::is_constructible<T1, T2>::value &&
                std::is_assignable<T1 &, T2>::value>::value,
        int>
        T3 = 0>
typename std::hash<std::optional<T1>>::argument_type &
f267(typename std::hash<std::__enable_hash_helper<
         std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
     T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename... T2,
          std::enable_if_t<std::is_constructible_v<T1, T2...>, int> T3 = 0>
T1 &f268(typename std::hash<std::__enable_hash_helper<
             std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
         T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <
    typename T1, typename T2, typename... T3,
    std::enable_if_t<
        std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, int>
        T4 = 0>
T1 &f269(typename std::hash<std::__enable_hash_helper<
             std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
         std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
T1 f270(const typename std::hash<std::__enable_hash_helper<
            std::optional<T1>, std::remove_const_t<T1>>>::argument_type &a0,
        T2 &&a1) {
  return a0.template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
T1 f271(typename std::hash<std::__enable_hash_helper<
            std::optional<T1>, std::remove_const_t<T1>>>::argument_type &&a0,
        T2 &&a1) {
  return std::move(a0).template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1>
using t60 = typename std::hash<std::shared_ptr<T1>>::argument_type::weak_type;

template <typename... T1>
using t61 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type;

template <typename... T1>
using t62 = typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::result_type;

template <typename... T1>
typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type
f272(const typename std::hash<std::variant<T1...>>::argument_type &a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type(a0);
}

template <typename... T1>
typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type
f273(typename std::hash<std::variant<T1...>>::argument_type &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::variant<T1...>,
      std::remove_const_t<T1>...>>::argument_type(std::move(a0));
}

template <typename... T1>
typename std::hash<std::variant<T1...>>::argument_type &
f274(typename std::hash<std::__enable_hash_helper<
         std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type &a0,
     const typename std::hash<std::variant<T1...>>::argument_type &a1) {
  return a0.operator=(a1);
}

template <typename... T1>
typename std::hash<std::variant<T1...>>::argument_type &
f275(typename std::hash<std::__enable_hash_helper<
         std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type &a0,
     typename std::hash<std::variant<T1...>>::argument_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
bool f276(
    const typename std::hash<std::__enable_hash_helper<
        std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type &a0) {
  return a0.valueless_by_exception();
}

template <typename... T1>
std::size_t
f277(const typename std::hash<std::__enable_hash_helper<
         std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type &a0) {
  return a0.index();
}

template <
    typename... T1,
    typename T2 = std::variant_alternative_t<
        0, typename std::hash<std::variant<T1...>>::argument_type>,
    std::enable_if_t<std::is_default_constructible_v<
                         std::variant_alternative_t<0, std::variant<T1...>>>,
                     int>
        T3 = 0>
typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type
f278() {
  return typename std::hash<std::__enable_hash_helper<
      std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type();
}

template <typename... T1, std::size_t T2, typename... T3,
          typename T4 = std::enable_if_t<(T2 < sizeof...(T1)), int>,
          typename T5 = std::variant_alternative_t<
              T2, typename std::hash<std::variant<T1...>>::argument_type>,
          std::enable_if_t<
              std::is_constructible_v<
                  std::variant_alternative_t<T2, std::variant<T1...>>, T3...>,
              int>
              T6 = 0>
typename std::hash<std::__enable_hash_helper<
    std::variant<T1...>, std::remove_const_t<T1>...>>::argument_type
f279(std::in_place_index_t<T2> a0, T3 &&...a1) {
  return typename std::hash<std::__enable_hash_helper<
      std::variant<T1...>,
      std::remove_const_t<T1>...>>::argument_type(a0, std::forward<T3>(a1)...);
}
