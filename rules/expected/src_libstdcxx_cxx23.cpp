#include <algorithm>
#include <expected>
#include <utility>

template <typename T1, typename T2>
std::expected<T1, T2> f69(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f70(const std::expected<T1, T2> &a0)
  requires(std::is_copy_constructible_v<T2> &&
           (!std::is_trivially_copy_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f71(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f72(std::expected<T1, T2> &&a0)
  requires(std::is_move_constructible_v<T2> &&
           (!std::is_trivially_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f73(std::expected<T1, T2> &a0,
                           const std::expected<T1, T2> &a1)
  requires(std::is_copy_constructible_v<T2> && std::is_copy_assignable_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f74(std::expected<T1, T2> &a0,
                           std::expected<T1, T2> &&a1)
  requires(std::is_move_constructible_v<T2> && std::is_move_assignable_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f75(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires(std::is_swappable_v<T2> && std::is_move_constructible_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f76(const std::expected<T3, T4> &a0)
  requires(std::is_void_v<T3> && std::is_constructible_v<T2, const T4 &> &&
           (!std::expected<T1, T2>::template __cons_from_expected<T3, T4>)) &&
          (std::is_void_v<T1>) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f77(std::expected<T3, T4> &&a0)
  requires(std::is_void_v<T3> && std::is_constructible_v<T2, T4> &&
           (!std::expected<T1, T2>::template __cons_from_expected<T3, T4>)) &&
          (std::is_void_v<T1>) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f78(std::expected<T1, T2> &a0,
                           const std::unexpected<T3> &a1)
  requires(std::is_constructible_v<T2, const T3 &> &&
           std::is_assignable_v<T2 &, const T3 &>) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f79(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires(std::is_constructible_v<T2, T3> && std::is_assignable_v<T2 &, T3>) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
auto f80(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f81(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f82(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f83(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
std::expected<T1, T2> f84(const std::expected<T1, T2> &a0) {
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f85(const std::expected<T1, T2> &a0)
  requires(std::is_copy_constructible_v<T1> &&
           std::is_copy_constructible_v<T2> &&
           (!std::is_trivially_copy_constructible_v<T1> ||
            !std::is_trivially_copy_constructible_v<T2>))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f86(std::expected<T1, T2> &&a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f87(std::expected<T1, T2> &&a0)
  requires(std::is_move_constructible_v<T1> &&
           std::is_move_constructible_v<T2> &&
           (!std::is_trivially_move_constructible_v<T1> ||
            !std::is_trivially_move_constructible_v<T2>))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f88(std::expected<T1, T2> &a0,
                           const std::expected<T1, T2> &a1)
  requires(std::is_copy_assignable_v<T1> && std::is_copy_constructible_v<T1> &&
           std::is_copy_assignable_v<T2> && std::is_copy_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f89(std::expected<T1, T2> &a0,
                           std::expected<T1, T2> &&a1)
  requires(std::is_move_assignable_v<T1> && std::is_move_constructible_v<T1> &&
           std::is_move_assignable_v<T2> && std::is_move_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f90(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires(std::is_swappable_v<T1> && std::is_swappable_v<T2> &&
           std::is_move_constructible_v<T1> &&
           std::is_move_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f91(const std::expected<T3, T4> &a0)
  requires(std::is_constructible_v<T1, const T3 &> &&
           std::is_constructible_v<T2, const T4 &> &&
           (!std::expected<T1, T2>::template __cons_from_expected<T3, T4>)) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f92(std::expected<T3, T4> &&a0)
  requires(std::is_constructible_v<T1, T3> && std::is_constructible_v<T2, T4> &&
           (!std::expected<T1, T2>::template __cons_from_expected<T3, T4>)) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f93(T3 &&a0)
  requires((!std::is_same_v<std::remove_cvref_t<T3>, std::expected<T1, T2>>) &&
           (!std::is_same_v<std::remove_cvref_t<T3>, std::in_place_t>) &&
           (!std::is_same_v<std::remove_cvref_t<T3>, std::unexpect_t>) &&
           std::is_constructible_v<T1, T3> &&
           (!std::__expected::__is_unexpected<std::remove_cvref_t<T3>>) &&
           std::__expected::__not_constructing_bool_from_expected<T1, T3>) &&
          (!(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return std::expected<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f94(std::expected<T1, T2> &a0, T3 &&a1)
  requires((!std::is_same_v<std::expected<T1, T2>, std::remove_cvref_t<T3>>) &&
           (!std::__expected::__is_unexpected<std::remove_cvref_t<T3>>) &&
           std::is_constructible_v<T1, T3> && std::is_assignable_v<T1 &, T3> &&
           (std::is_nothrow_constructible_v<T1, T3> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>)) &&
          (!(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f95(std::expected<T1, T2> &a0,
                           const std::unexpected<T3> &a1)
  requires(std::is_constructible_v<T2, const T3 &> &&
           std::is_assignable_v<T2 &, const T3 &> &&
           (std::is_nothrow_constructible_v<T2, const T3 &> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f96(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires(std::is_constructible_v<T2, T3> && std::is_assignable_v<T2 &, T3> &&
           (std::is_nothrow_constructible_v<T2, T3> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
auto f97(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f98(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f99(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f100(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f101(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f102(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f103(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f104(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

using t6 = std::unexpect_t;

std::unexpect_t f105() { return std::unexpect_t(); }

std::unexpect_t f106(const std::unexpect_t &a0) { return std::unexpect_t(a0); }

std::unexpect_t f107(std::unexpect_t &&a0) {
  return std::unexpect_t(std::move(a0));
}

std::unexpect_t &f108(std::unexpect_t &a0, const std::unexpect_t &a1) {
  return a0.operator=(a1);
}

std::unexpect_t &f109(std::unexpect_t &a0, std::unexpect_t &&a1) {
  return a0.operator=(std::move(a1));
}
