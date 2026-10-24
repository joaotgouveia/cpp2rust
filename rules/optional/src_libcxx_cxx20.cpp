#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f404(const std::optional<T1> &a0,
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
std::optional<T1> f405(T2 &&a0)
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
std::optional<T1> f406(T2 &&a0)
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
std::optional<T1> f407(const std::optional<T2> &a0)
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
std::optional<T1> f408(const std::optional<T2> &a0)
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
std::optional<T1> f409(std::optional<T2> &&a0)
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
std::optional<T1> f410(std::optional<T2> &&a0)
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
std::optional<T1> &f411(std::optional<T1> &a0, T2 &&a1)
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
std::optional<T1> &f412(std::optional<T1> &a0, const std::optional<T2> &a1)
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
std::optional<T1> &f413(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
void f414(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
bool f415(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f416(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
T1 &f417(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator*();
}

template <typename T1>
T1 &f418(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.value();
}

template <typename T1>
bool f419(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f420(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f421(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f422(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f423(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f424(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f425(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f426(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f427(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f428(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f429(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f430(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f431(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f432(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f433(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f434(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f435(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f436(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f437(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f438(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f439(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f440(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f441(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f442(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f443(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f444(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f445(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f446(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f447(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f448(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f449(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f450(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f451(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f452(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f453(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f454(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f455(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f456(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f457(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f458(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f459(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f460(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f461(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f462(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f463(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f464(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f465(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f466(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f467(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f468(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f469(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f470(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f471(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f472(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f473(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f474(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f475(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f476(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f477(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f478(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f479(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f480(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f481(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f482(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f483(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f484(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f485(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f486(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f487(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f488(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f489(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f490(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f491(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f492(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f493(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f494(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f495(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f496(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f497(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f498(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f499(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f500(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f501(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f502(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f503(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f504(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f505(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f506(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f507(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f508(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f509(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f510(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f511(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f512(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f513(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f514(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f515(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f516(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f517(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f518(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f519(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f520(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f521(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f522(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f523(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f524(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f525(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f526(std::optional<T1> &a0)
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
bool f527(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f528(const std::optional<T1> &a0)
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
T1 &f529(const std::optional<T1> &a0)
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
T1 &f530(const std::optional<T1> &a0)
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
bool f531(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f532(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f533(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f534(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f535(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f536(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f537(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f538(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f539(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f540(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f541(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f542(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f543(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f544(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f545(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f546(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f547(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f548(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f549(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f550(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f551(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f552(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f553(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f554(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f555(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f556(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f557(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f558(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f559(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f560(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f561(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f562(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f563(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f564(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f565(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f566(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f567(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f568(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f569(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f570(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f571(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f572(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f573(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f574(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f575(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f576(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f577(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f578(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f579(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f580(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f581(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f582(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f583(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f584(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f585(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f586(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f587(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f588(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f589(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f590(std::optional<T1> &a0)
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
bool f591(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f592(const std::optional<T1> &a0)
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
T1 &f593(const std::optional<T1> &a0)
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
T1 &f594(const std::optional<T1> &a0)
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
bool f595(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f596(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f597(std::optional<T1> &a0)
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
T1 const &f598(const std::optional<T1> &a0)
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
T1 &f599(std::optional<T1> &a0)
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
T1 &&f600(std::optional<T1> &&a0)
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
T1 const &&f601(const std::optional<T1> &&a0)
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
T1 const &f602(const std::optional<T1> &a0)
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
T1 &f603(std::optional<T1> &a0)
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
T1 &&f604(std::optional<T1> &&a0)
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
T1 const &&f605(const std::optional<T1> &&a0)
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
void f606(std::optional<T1> &a0)
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
bool f607(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f608(const std::optional<T1> &a0)
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
T1 &f609(const std::optional<T1> &a0)
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
T1 &f610(const std::optional<T1> &a0)
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
bool f611(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f612(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f613(std::optional<T1> &a0)
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
T1 const &f614(const std::optional<T1> &a0)
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
T1 &f615(std::optional<T1> &a0)
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
T1 &&f616(std::optional<T1> &&a0)
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
T1 const &&f617(const std::optional<T1> &&a0)
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
T1 const &f618(const std::optional<T1> &a0)
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
T1 &f619(std::optional<T1> &a0)
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
T1 &&f620(std::optional<T1> &&a0)
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
T1 const &&f621(const std::optional<T1> &&a0)
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
void f622(std::optional<T1> &a0)
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
bool f623(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f624(const std::optional<T1> &a0)
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
  return a0.operator*();
}

template <typename T1>
T1 &f626(const std::optional<T1> &a0)
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
bool f627(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f628(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f629(std::optional<T1> &a0)
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
T1 const &f630(const std::optional<T1> &a0)
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
T1 &f631(std::optional<T1> &a0)
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
T1 &&f632(std::optional<T1> &&a0)
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
T1 const &&f633(const std::optional<T1> &&a0)
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
T1 const &f634(const std::optional<T1> &a0)
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
T1 &f635(std::optional<T1> &a0)
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
T1 &&f636(std::optional<T1> &&a0)
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
T1 const &&f637(const std::optional<T1> &&a0)
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
void f638(std::optional<T1> &a0)
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
bool f639(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f640(const std::optional<T1> &a0)
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
  return a0.operator*();
}

template <typename T1>
T1 &f642(const std::optional<T1> &a0)
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
bool f643(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f644(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f645(std::optional<T1> &a0)
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
T1 const &f646(const std::optional<T1> &a0)
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
T1 &f647(std::optional<T1> &a0)
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
T1 &&f648(std::optional<T1> &&a0)
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
T1 const &&f649(const std::optional<T1> &&a0)
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
T1 const &f650(const std::optional<T1> &a0)
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
T1 &f651(std::optional<T1> &a0)
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
T1 &&f652(std::optional<T1> &&a0)
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
T1 const &&f653(const std::optional<T1> &&a0)
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
void f654(std::optional<T1> &a0)
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
bool f655(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f656(const std::optional<T1> &a0)
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
  return a0.operator*();
}

template <typename T1>
T1 &f658(const std::optional<T1> &a0)
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
