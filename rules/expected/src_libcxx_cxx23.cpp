#include <expected>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::expected<T1, T2> f110(const std::expected<T1, T2> &a0)
  requires((std::is_copy_constructible_v<T2> &&
            std::is_trivially_copy_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f111(const std::expected<T1, T2> &a0)
  requires((std::is_copy_constructible_v<T2> &&
            !std::is_trivially_copy_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f112(std::expected<T1, T2> &&a0)
  requires((std::is_move_constructible_v<T2> &&
            std::is_trivially_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f113(std::expected<T1, T2> &&a0)
  requires((std::is_move_constructible_v<T2> &&
            !std::is_trivially_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f114(std::expected<T1, T2> &a0,
                            const std::expected<T1, T2> &a1)
  requires((std::is_copy_assignable_v<T2> &&
            std::is_copy_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f115(std::expected<T1, T2> &a0,
                            std::expected<T1, T2> &&a1)
  requires((std::is_move_assignable_v<T2> &&
            std::is_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f116(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires((std::is_swappable_v<T2> && std::is_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f117(const std::expected<T3, T4> &a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f118(std::expected<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f119(std::expected<T1, T2> &a0,
                            const std::unexpected<T3> &a1)
  requires((std::is_constructible_v<T2, const T3 &> &&
            std::is_assignable_v<T2 &, const T3 &>)) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f120(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires((std::is_constructible_v<T2, T3> &&
            std::is_assignable_v<T2 &, T3>)) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
auto f121(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &&>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f122(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &&>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f123(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &&>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f124(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &&>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
std::expected<T1, T2> f125(const std::expected<T1, T2> &a0)
  requires((std::is_copy_constructible_v<T1> &&
            std::is_copy_constructible_v<T2> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T2>))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f126(const std::expected<T1, T2> &a0)
  requires((std::is_copy_constructible_v<T1> &&
            std::is_copy_constructible_v<T2> &&
            !(std::is_trivially_copy_constructible_v<T1> &&
              std::is_trivially_copy_constructible_v<T2>)))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f127(std::expected<T1, T2> &&a0)
  requires((std::is_move_constructible_v<T1> &&
            std::is_move_constructible_v<T2> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_constructible_v<T2>))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f128(std::expected<T1, T2> &&a0)
  requires((std::is_move_constructible_v<T1> &&
            std::is_move_constructible_v<T2> &&
            !(std::is_trivially_move_constructible_v<T1> &&
              std::is_trivially_move_constructible_v<T2>)))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f129(std::expected<T1, T2> &a0,
                            const std::expected<T1, T2> &a1)
  requires((std::is_copy_assignable_v<T1> && std::is_copy_constructible_v<T1> &&
            std::is_copy_assignable_v<T2> && std::is_copy_constructible_v<T2> &&
            (std::is_nothrow_move_constructible_v<T1> ||
             std::is_nothrow_move_constructible_v<T2>)))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f130(std::expected<T1, T2> &a0,
                            std::expected<T1, T2> &&a1)
  requires((std::is_move_constructible_v<T1> && std::is_move_assignable_v<T1> &&
            std::is_move_constructible_v<T2> && std::is_move_assignable_v<T2> &&
            (std::is_nothrow_move_constructible_v<T1> ||
             std::is_nothrow_move_constructible_v<T2>)))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f131(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires((std::is_swappable_v<T1> && std::is_swappable_v<T2> &&
            std::is_move_constructible_v<T1> &&
            std::is_move_constructible_v<T2> &&
            (std::is_nothrow_move_constructible_v<T1> ||
             std::is_nothrow_move_constructible_v<T2>)))
{
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f132(T3 &&a0)
  requires((!std::is_same_v<std::remove_cvref_t<T3>, std::in_place_t> &&
            !std::is_same_v<std::expected<T1, T2>, std::remove_cvref_t<T3>> &&
            !std::is_same_v<std::remove_cvref_t<T3>, std::unexpect_t> &&
            std::is_constructible_v<T1, T3> &&
            !std::__is_std_unexpected<std::remove_cvref_t<T3>>::value &&
            (!std::is_same_v<std::remove_cv_t<T1>, bool> ||
             !std::__is_std_expected<std::remove_cvref_t<T3>>::value))) &&
          (!(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return std::expected<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f133(std::expected<T1, T2> &a0, T3 &&a1)
  requires((!std::is_same_v<std::expected<T1, T2>, std::remove_cvref_t<T3>> &&
            !std::__is_std_unexpected<std::remove_cvref_t<T3>>::value &&
            std::is_constructible_v<T1, T3> && std::is_assignable_v<T1 &, T3> &&
            (std::is_nothrow_constructible_v<T1, T3> ||
             std::is_nothrow_move_constructible_v<T1> ||
             std::is_nothrow_move_constructible_v<T2>))) &&
          (!(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f134(std::expected<T1, T2> &a0,
                            const std::unexpected<T3> &a1)
  requires((
      std::expected<T1, T2>::template __can_assign_from_unexpected<const T3 &>))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f135(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires((std::expected<T1, T2>::template __can_assign_from_unexpected<T3>))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
auto f136(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &&>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f137(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &&>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f138(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &&>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f139(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &&>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f140(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &&>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f141(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &&>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f142(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &&>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f143(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &&>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}
