#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f67(const std::optional<T1> &a0,
                                            const T2 &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::optional<T1> f68(T2 &&a0)
  requires((!std::is_same_v<std::optional<T1>, std::remove_cvref_t<T2>>) &&
           (!std::is_same_v<std::in_place_t, std::remove_cvref_t<T2>>) &&
           std::is_constructible_v<T1, T2> &&
           std::optional<T1>::template __not_constructing_bool_from_optional<
               T2>) &&
          (!(std::is_same<T2, std::nullopt_t>::value))
{
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
std::optional<T1> f69(const std::optional<T2> &a0)
  requires((!std::is_same_v<T1, T2>) &&
           std::is_constructible_v<T1, const T2 &> &&
           std::optional<T1>::template __construct_from_contained_value<T2>)
{
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f70(std::optional<T2> &&a0)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2> &&
           std::optional<T1>::template __construct_from_contained_value<T2>)
{
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2>
std::optional<T1> f71(std::in_place_t a0, T2 &&...a1)
  requires(std::is_constructible_v<T1, T2...>)
{
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f72(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2)
  requires(std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>)
{
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> &f73(std::optional<T1> &a0, T2 &&a1)
  requires((!std::is_same_v<std::optional<T1>, std::remove_cvref_t<T2>>) &&
           (!(std::is_scalar_v<T1> && std::is_same_v<T1, std::decay_t<T2>>)) &&
           std::is_constructible_v<T1, T2> && std::is_assignable_v<T1 &, T2>) &&
          (!(std::is_same<T2, std::nullopt_t>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f74(std::optional<T1> &a0, const std::optional<T2> &a1)
  requires((!std::is_same_v<T1, T2>) &&
           std::is_constructible_v<T1, const T2 &> &&
           std::is_assignable_v<T1 &, const T2 &> &&
           (!std::__converts_from_optional<T1, T2>::value) &&
           (!std::__assigns_from_optional<T1, T2>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f75(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2> &&
           std::is_assignable_v<T1 &, T2> &&
           (!std::__converts_from_optional<T1, T2>::value) &&
           (!std::__assigns_from_optional<T1, T2>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}
