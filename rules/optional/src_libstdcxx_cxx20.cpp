#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::__optional_ne_t<T1, T2> f85(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ne_t<T1, T2> f86(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_lt_t<T1, T2> f87(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_lt_t<T1, T2> f88(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_le_t<T1, T2> f89(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_le_t<T1, T2> f90(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f91(const std::optional<T1> &a0,
                                            const T2 &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_eq_t<T1, T2> f92(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_eq_t<T1, T2> f93(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_gt_t<T1, T2> f94(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_gt_t<T1, T2> f95(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ge_t<T1, T2> f96(const std::optional<T1> &a0, const T2 &a1)
  requires((!std::__is_optional_v<T2>))
{
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::__optional_ge_t<T1, T2> f97(const T1 &a0, const std::optional<T2> &a1)
  requires((!std::__is_optional_v<T1>))
{
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::optional<T1> f98(T2 &&a0)
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
std::optional<T1> f99(const std::optional<T2> &a0)
  requires((!std::is_same_v<T1, T2>) &&
           std::is_constructible_v<T1, const T2 &> &&
           std::optional<T1>::template __construct_from_contained_value<T2>)
{
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f100(std::optional<T2> &&a0)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2> &&
           std::optional<T1>::template __construct_from_contained_value<T2>)
{
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2>
std::optional<T1> f101(std::in_place_t a0, T2 &&...a1)
  requires(std::is_constructible_v<T1, T2...>)
{
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f102(std::in_place_t a0, std::initializer_list<T2> a1,
                       T3 &&...a2)
  requires(std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>)
{
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> &f103(std::optional<T1> &a0, T2 &&a1)
  requires((!std::is_same_v<std::optional<T1>, std::remove_cvref_t<T2>>) &&
           (!(std::is_scalar_v<T1> && std::is_same_v<T1, std::decay_t<T2>>)) &&
           std::is_constructible_v<T1, T2> && std::is_assignable_v<T1 &, T2>) &&
          (!(std::is_same<T2, std::nullopt_t>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f104(std::optional<T1> &a0, const std::optional<T2> &a1)
  requires((!std::is_same_v<T1, T2>) &&
           std::is_constructible_v<T1, const T2 &> &&
           std::is_assignable_v<T1 &, const T2 &> &&
           (!std::__converts_from_optional<T1, T2>::value) &&
           (!std::__assigns_from_optional<T1, T2>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f105(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2> &&
           std::is_assignable_v<T1 &, T2> &&
           (!std::__converts_from_optional<T1, T2>::value) &&
           (!std::__assigns_from_optional<T1, T2>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}
