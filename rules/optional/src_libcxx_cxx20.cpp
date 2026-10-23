#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f370(const std::optional<T1> &a0,
                                             const T2 &a1)
  requires((!std::__is_derived_from_optional<T2>) &&
           std::three_way_comparable_with<T1, T2>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<
            std::_IsNotSame<std::__remove_cvref_t<T2>,
                            std::in_place_t>::value &&
                std::_IsNotSame<std::__remove_cvref_t<T2>,
                                std::optional<T1>>::value &&
                (!std::is_same_v<std::remove_cv_t<T1>, bool> ||
                 !std::__is_std_optional<std::__remove_cvref_t<T2>>::value),
            typename std::optional<T1>::_CheckOptionalArgsConstructor,
            std::__check_tuple_constructor_fail>::
            template __enable_implicit<T2>(),
        int>
        T3 = 0>
std::optional<T1> f371(T2 &&a0)
  requires(!(std::is_same<T2, const std::optional<T1> &>::value) &&
           !(std::is_same<T2, std::optional<T1>>::value) &&
           !(std::is_same<T2, std::nullopt_t>::value))
{
  return std::optional<T1>(std::forward<T2>(a0));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<
            std::_IsNotSame<std::__remove_cvref_t<T2>,
                            std::in_place_t>::value &&
                std::_IsNotSame<std::__remove_cvref_t<T2>,
                                std::optional<T1>>::value &&
                (!std::is_same_v<std::remove_cv_t<T1>, bool> ||
                 !std::__is_std_optional<std::__remove_cvref_t<T2>>::value),
            typename std::optional<T1>::_CheckOptionalArgsConstructor,
            std::__check_tuple_constructor_fail>::
            template __enable_explicit<T2>(),
        int>
        T3 = 0>
std::optional<T1> f372(T2 &&a0)
  requires(!(std::is_same<T2, const std::optional<T1> &>::value) &&
           !(std::is_same<T2, std::optional<T1>>::value) &&
           !(std::is_same<T2, std::nullopt_t>::value))
{
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, const T2 &>>::value,
                       typename std::optional<T1>::
                           template _CheckOptionalLikeConstructor<const T2 &>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_implicit<T2>(),
              int>
              T3 = 0>
std::optional<T1> f373(const std::optional<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::optional<T1>(a0);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, const T2 &>>::value,
                       typename std::optional<T1>::
                           template _CheckOptionalLikeConstructor<const T2 &>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_explicit<T2>(),
              int>
              T3 = 0>
std::optional<T1> f374(const std::optional<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::optional<T1>(a0);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, T2 &&>>::value,
                       typename std::optional<
                           T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_implicit<T2>(),
              int>
              T3 = 0>
std::optional<T1> f375(std::optional<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, T2 &&>>::value,
                       typename std::optional<
                           T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_explicit<T2>(),
              int>
              T3 = 0>
std::optional<T1> f376(std::optional<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::optional<T1>(std::move(a0));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_And<std::_IsNotSame<std::__remove_cvref_t<T2>, std::optional<T1>>,
                  std::_Or<std::_IsNotSame<std::__remove_cvref_t<T2>, T1>,
                           std::_Not<std::is_scalar<T1>>>,
                  std::is_constructible<T1, T2>,
                  std::is_assignable<T1 &, T2>>::value,
        int>
        T3 = 0>
std::optional<T1> &f377(std::optional<T1> &a0, T2 &&a1)
  requires(!(std::is_same<T2, std::nullopt_t>::value) &&
           !(std::is_same<T2, const std::optional<T1> &>::value) &&
           !(std::is_same<T2, std::optional<T1>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, const T2 &>,
                                 std::is_assignable<T1 &, const T2 &>>::value,
                       typename std::optional<T1>::
                           template _CheckOptionalLikeConstructor<const T2 &>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_assign<T2>(),
              int>
              T3 = 0>
std::optional<T1> &f378(std::optional<T1> &a0, const std::optional<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2,
          std::enable_if_t<
              std::_If<std::_And<std::_IsNotSame<T2, T1>,
                                 std::is_constructible<T1, T2 &&>>::value,
                       typename std::optional<
                           T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                       std::__check_tuple_constructor_fail>::
                  template __enable_assign<T2>(),
              int>
              T3 = 0>
std::optional<T1> &f379(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
void f380(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
bool f381(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f382(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
T1 &f383(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator*();
}

template <typename T1>
T1 &f384(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.value();
}

template <typename T1>
bool f385(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f386(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f387(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f388(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f389(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f390(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f391(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f392(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f393(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f394(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f395(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f396(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f397(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f398(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f399(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f400(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f401(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f402(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f403(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f404(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f405(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f406(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f407(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f408(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f409(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f410(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f411(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f412(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f413(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f414(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f415(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f416(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f417(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f418(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f419(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f420(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f421(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f422(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f423(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f424(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f425(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f426(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f427(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f428(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f429(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f430(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f431(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f432(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f433(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f434(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f435(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f436(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f437(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f438(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f439(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f440(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f441(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f442(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f443(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f444(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f445(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f446(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f447(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f448(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f449(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f450(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f451(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f452(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f453(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f454(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f455(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f456(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f457(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f458(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f459(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f460(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f461(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f462(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f463(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f464(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f465(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f466(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f467(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f468(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f469(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f470(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f471(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f472(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f473(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f474(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f475(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f476(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f477(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f478(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f479(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f480(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f481(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f482(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f483(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f484(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f485(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f486(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f487(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f488(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f489(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f490(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f491(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f492(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f493(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f494(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f495(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f496(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f497(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f498(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f499(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f500(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f501(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f502(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f503(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f504(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f505(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f506(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f507(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f508(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f509(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f510(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f511(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f512(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f513(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f514(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f515(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f516(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f517(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f518(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f519(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f520(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f521(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f522(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f523(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f524(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f525(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f526(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f527(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f528(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f529(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f530(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f531(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f532(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f533(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f534(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f535(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f536(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f537(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f538(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f539(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f540(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f541(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f542(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f543(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f544(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f545(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f546(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f547(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f548(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f549(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f550(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f551(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f552(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f553(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f554(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f555(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f556(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f557(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f558(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f559(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f560(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f561(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f562(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f563(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f564(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f565(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f566(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f567(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f568(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f569(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f570(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f571(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f572(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f573(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f574(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f575(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f576(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f577(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f578(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f579(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f580(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f581(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f582(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f583(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f584(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f585(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f586(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f587(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f588(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f589(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f590(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f591(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f592(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f593(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f594(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f595(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f596(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f597(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f598(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f599(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f600(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f601(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f602(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f603(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f604(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f605(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f606(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f607(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f608(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f609(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f610(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f611(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f612(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f613(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f614(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f615(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f616(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f617(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f618(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f619(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f620(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f621(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f622(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f623(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f624(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}
