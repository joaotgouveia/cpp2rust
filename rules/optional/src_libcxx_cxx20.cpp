#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f373(const std::optional<T1> &a0,
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
std::optional<T1> f374(T2 &&a0)
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
std::optional<T1> f375(T2 &&a0)
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
std::optional<T1> f376(const std::optional<T2> &a0)
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
std::optional<T1> f377(const std::optional<T2> &a0)
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
std::optional<T1> f378(std::optional<T2> &&a0)
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
std::optional<T1> f379(std::optional<T2> &&a0)
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
std::optional<T1> &f380(std::optional<T1> &a0, T2 &&a1)
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
std::optional<T1> &f381(std::optional<T1> &a0, const std::optional<T2> &a1)
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
std::optional<T1> &f382(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
void f383(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
bool f384(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f385(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
T1 &f386(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator*();
}

template <typename T1>
T1 &f387(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.value();
}

template <typename T1>
bool f388(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f389(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f390(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f391(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f392(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f393(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f394(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f395(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f396(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f397(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f398(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f399(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f400(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f401(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f402(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f403(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f404(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f405(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f406(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f407(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f408(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f409(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f410(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f411(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f412(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f413(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f414(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f415(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f416(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f417(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f418(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f419(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f420(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f421(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f422(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f423(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f424(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f425(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f426(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f427(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f428(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f429(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f430(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f431(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f432(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f433(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f434(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f435(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f436(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f437(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f438(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f439(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f440(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f441(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f442(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f443(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f444(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f445(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f446(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f447(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f448(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f449(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f450(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f451(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f452(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f453(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f454(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f455(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f456(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f457(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f458(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f459(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f460(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f461(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f462(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f463(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f464(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f465(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f466(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f467(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f468(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f469(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f470(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f471(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f472(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f473(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f474(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f475(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f476(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f477(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f478(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f479(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f480(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f481(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f482(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f483(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f484(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f485(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f486(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f487(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f488(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f489(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f490(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f491(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f492(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f493(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f494(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f495(std::optional<T1> &a0)
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
bool f496(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f497(const std::optional<T1> &a0)
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
T1 &f498(const std::optional<T1> &a0)
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
T1 &f499(const std::optional<T1> &a0)
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
bool f500(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f501(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f502(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f503(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f504(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f505(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f506(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f507(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f508(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f509(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f510(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f511(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f512(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f513(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f514(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f515(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f516(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f517(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f518(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f519(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f520(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f521(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f522(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f523(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f524(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f525(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f526(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f527(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f528(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f529(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f530(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f531(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f532(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f533(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f534(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f535(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f536(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f537(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f538(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f539(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f540(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f541(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f542(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f543(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f544(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f545(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f546(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f547(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f548(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f549(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f550(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f551(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f552(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f553(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f554(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f555(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f556(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f557(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f558(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f559(std::optional<T1> &a0)
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
bool f560(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f561(const std::optional<T1> &a0)
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
T1 &f562(const std::optional<T1> &a0)
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
T1 &f563(const std::optional<T1> &a0)
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
bool f564(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f565(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f566(std::optional<T1> &a0)
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
T1 const &f567(const std::optional<T1> &a0)
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
T1 &f568(std::optional<T1> &a0)
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
T1 &&f569(std::optional<T1> &&a0)
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
T1 const &&f570(const std::optional<T1> &&a0)
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
T1 const &f571(const std::optional<T1> &a0)
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
T1 &f572(std::optional<T1> &a0)
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
T1 &&f573(std::optional<T1> &&a0)
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
T1 const &&f574(const std::optional<T1> &&a0)
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
void f575(std::optional<T1> &a0)
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
bool f576(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f577(const std::optional<T1> &a0)
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
T1 &f578(const std::optional<T1> &a0)
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
T1 &f579(const std::optional<T1> &a0)
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
bool f580(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f581(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f582(std::optional<T1> &a0)
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
T1 const &f583(const std::optional<T1> &a0)
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
T1 &f584(std::optional<T1> &a0)
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
T1 &&f585(std::optional<T1> &&a0)
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
T1 const &&f586(const std::optional<T1> &&a0)
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
T1 const &f587(const std::optional<T1> &a0)
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
T1 &f588(std::optional<T1> &a0)
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
T1 &&f589(std::optional<T1> &&a0)
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
T1 const &&f590(const std::optional<T1> &&a0)
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
void f591(std::optional<T1> &a0)
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
bool f592(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f593(const std::optional<T1> &a0)
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
T1 &f594(const std::optional<T1> &a0)
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
T1 &f595(const std::optional<T1> &a0)
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
bool f596(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f597(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f598(std::optional<T1> &a0)
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
T1 const &f599(const std::optional<T1> &a0)
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
T1 &f600(std::optional<T1> &a0)
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
T1 &&f601(std::optional<T1> &&a0)
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
T1 const &&f602(const std::optional<T1> &&a0)
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
T1 const &f603(const std::optional<T1> &a0)
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
T1 &f604(std::optional<T1> &a0)
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
T1 &&f605(std::optional<T1> &&a0)
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
T1 const &&f606(const std::optional<T1> &&a0)
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
void f607(std::optional<T1> &a0)
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
bool f608(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f609(const std::optional<T1> &a0)
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
T1 &f610(const std::optional<T1> &a0)
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
T1 &f611(const std::optional<T1> &a0)
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
bool f612(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f613(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f614(std::optional<T1> &a0)
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
T1 const &f615(const std::optional<T1> &a0)
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
T1 &f616(std::optional<T1> &a0)
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
T1 &&f617(std::optional<T1> &&a0)
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
T1 const &&f618(const std::optional<T1> &&a0)
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
T1 const &f619(const std::optional<T1> &a0)
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
T1 &f620(std::optional<T1> &a0)
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
T1 &&f621(std::optional<T1> &&a0)
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
T1 const &&f622(const std::optional<T1> &&a0)
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
void f623(std::optional<T1> &a0)
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
bool f624(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f625(const std::optional<T1> &a0)
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
T1 &f626(const std::optional<T1> &a0)
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
T1 &f627(const std::optional<T1> &a0)
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
