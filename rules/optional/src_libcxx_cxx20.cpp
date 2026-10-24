#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f403(const std::optional<T1> &a0,
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
std::optional<T1> f404(T2 &&a0)
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
std::optional<T1> f405(T2 &&a0)
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
std::optional<T1> f406(const std::optional<T2> &a0)
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
std::optional<T1> f407(const std::optional<T2> &a0)
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
std::optional<T1> f408(std::optional<T2> &&a0)
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
std::optional<T1> f409(std::optional<T2> &&a0)
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
std::optional<T1> &f410(std::optional<T1> &a0, T2 &&a1)
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
std::optional<T1> &f411(std::optional<T1> &a0, const std::optional<T2> &a1)
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
std::optional<T1> &f412(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
void f413(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
bool f414(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f415(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
T1 &f416(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator*();
}

template <typename T1>
T1 &f417(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.value();
}

template <typename T1>
bool f418(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f419(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f420(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f421(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f422(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f423(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f424(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f425(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f426(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f427(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f428(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f429(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f430(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f431(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f432(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f433(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f434(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f435(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f436(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f437(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f438(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f439(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f440(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f441(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f442(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f443(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f444(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f445(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f446(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f447(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f448(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f449(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f450(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f451(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f452(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f453(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f454(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f455(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f456(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f457(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f458(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f459(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f460(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f461(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f462(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f463(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f464(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f465(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f466(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f467(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f468(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f469(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f470(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f471(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f472(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f473(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f474(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f475(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f476(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f477(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f478(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f479(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f480(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f481(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f482(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f483(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f484(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f485(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f486(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f487(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f488(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f489(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f490(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f491(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f492(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f493(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f494(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f495(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f496(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f497(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f498(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f499(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f500(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f501(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f502(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f503(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f504(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f505(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f506(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f507(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f508(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f509(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f510(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f511(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f512(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f513(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f514(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f515(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f516(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f517(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f518(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f519(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f520(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f521(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f522(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f523(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f524(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f525(std::optional<T1> &a0)
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
bool f526(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f527(const std::optional<T1> &a0)
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
T1 &f528(const std::optional<T1> &a0)
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
T1 &f529(const std::optional<T1> &a0)
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
bool f530(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f531(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f532(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f533(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f534(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f535(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f536(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f537(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f538(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f539(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f540(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f541(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f542(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f543(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f544(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f545(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f546(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f547(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f548(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f549(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f550(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f551(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f552(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f553(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f554(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f555(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f556(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f557(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f558(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f559(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f560(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f561(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f562(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f563(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f564(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f565(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f566(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f567(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f568(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f569(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f570(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f571(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f572(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f573(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f574(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f575(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f576(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f577(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f578(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f579(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f580(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f581(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f582(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f583(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f584(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f585(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f586(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f587(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f588(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f589(std::optional<T1> &a0)
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
bool f590(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f591(const std::optional<T1> &a0)
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
T1 &f592(const std::optional<T1> &a0)
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
T1 &f593(const std::optional<T1> &a0)
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
bool f594(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f595(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f596(std::optional<T1> &a0)
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
T1 const &f597(const std::optional<T1> &a0)
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
T1 &f598(std::optional<T1> &a0)
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
T1 &&f599(std::optional<T1> &&a0)
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
T1 const &&f600(const std::optional<T1> &&a0)
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
T1 const &f601(const std::optional<T1> &a0)
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
T1 &f602(std::optional<T1> &a0)
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
T1 &&f603(std::optional<T1> &&a0)
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
T1 const &&f604(const std::optional<T1> &&a0)
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
void f605(std::optional<T1> &a0)
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
bool f606(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f607(const std::optional<T1> &a0)
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
T1 &f608(const std::optional<T1> &a0)
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
T1 &f609(const std::optional<T1> &a0)
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
bool f610(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f611(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f612(std::optional<T1> &a0)
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
T1 const &f613(const std::optional<T1> &a0)
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
T1 &f614(std::optional<T1> &a0)
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
T1 &&f615(std::optional<T1> &&a0)
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
T1 const &&f616(const std::optional<T1> &&a0)
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
T1 const &f617(const std::optional<T1> &a0)
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
T1 &f618(std::optional<T1> &a0)
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
T1 &&f619(std::optional<T1> &&a0)
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
T1 const &&f620(const std::optional<T1> &&a0)
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
void f621(std::optional<T1> &a0)
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
bool f622(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f623(const std::optional<T1> &a0)
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
T1 &f624(const std::optional<T1> &a0)
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
T1 &f625(const std::optional<T1> &a0)
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
bool f626(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f627(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f628(std::optional<T1> &a0)
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
T1 const &f629(const std::optional<T1> &a0)
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
T1 &f630(std::optional<T1> &a0)
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
T1 &&f631(std::optional<T1> &&a0)
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
T1 const &&f632(const std::optional<T1> &&a0)
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
T1 const &f633(const std::optional<T1> &a0)
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
T1 &f634(std::optional<T1> &a0)
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
T1 &&f635(std::optional<T1> &&a0)
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
T1 const &&f636(const std::optional<T1> &&a0)
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
void f637(std::optional<T1> &a0)
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
bool f638(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f639(const std::optional<T1> &a0)
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
T1 &f640(const std::optional<T1> &a0)
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
T1 &f641(const std::optional<T1> &a0)
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
bool f642(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f643(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f644(std::optional<T1> &a0)
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
T1 const &f645(const std::optional<T1> &a0)
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
T1 &f646(std::optional<T1> &a0)
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
T1 &&f647(std::optional<T1> &&a0)
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
T1 const &&f648(const std::optional<T1> &&a0)
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
T1 const &f649(const std::optional<T1> &a0)
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
T1 &f650(std::optional<T1> &a0)
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
T1 &&f651(std::optional<T1> &&a0)
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
T1 const &&f652(const std::optional<T1> &&a0)
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
void f653(std::optional<T1> &a0)
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
bool f654(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f655(const std::optional<T1> &a0)
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
T1 &f656(const std::optional<T1> &a0)
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
T1 &f657(const std::optional<T1> &a0)
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
