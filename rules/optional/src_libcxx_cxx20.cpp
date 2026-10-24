#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f437(const std::optional<T1> &a0,
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
std::optional<T1> f438(T2 &&a0)
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
std::optional<T1> f439(T2 &&a0)
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
std::optional<T1> f440(const std::optional<T2> &a0)
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
std::optional<T1> f441(const std::optional<T2> &a0)
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
std::optional<T1> f442(std::optional<T2> &&a0)
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
std::optional<T1> f443(std::optional<T2> &&a0)
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
std::optional<T1> &f444(std::optional<T1> &a0, T2 &&a1)
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
std::optional<T1> &f445(std::optional<T1> &a0, const std::optional<T2> &a1)
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
std::optional<T1> &f446(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
void f447(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false)
{
  return a0.reset();
}

template <typename T1>
void f448(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
void f449(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.reset();
}

template <typename T1>
bool f450(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f451(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
T1 &f452(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.operator*();
}

template <typename T1>
T1 &f453(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true)
{
  return a0.value();
}

template <typename T1>
bool f454(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f455(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f456(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f457(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f458(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f459(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f460(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f461(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f462(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f463(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f464(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f465(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f466(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f467(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f468(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f469(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f470(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f471(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f472(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f473(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f474(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f475(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f476(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f477(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f478(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f479(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f480(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f481(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f482(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f483(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f484(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f485(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f486(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f487(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f488(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f489(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f490(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f491(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f492(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f493(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f494(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f495(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f496(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f497(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &f498(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f499(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f500(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f501(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f502(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
void f503(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.reset();
}

template <typename T1>
bool f504(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f505(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f506(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f507(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false)
{
  return a0.value();
}

template <typename T1>
bool f508(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f509(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f510(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f511(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f512(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f513(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f514(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f515(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f516(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f517(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f518(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f519(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f520(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f521(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f522(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f523(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f524(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f525(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f526(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f527(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f528(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f529(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f530(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f531(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f532(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f533(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f534(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f535(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f536(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f537(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f538(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f539(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f540(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f541(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f542(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f543(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f544(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f545(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f546(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f547(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f548(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f549(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f550(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f551(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f552(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f553(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f554(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f555(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f556(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f557(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f558(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f559(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f560(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f561(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f562(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f563(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f564(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f565(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f566(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f567(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f568(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f569(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f570(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f571(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f572(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f573(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f574(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_copy_constructible_v<T1> &&
            std::is_trivially_copy_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f575(std::optional<T1> &a0)
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
bool f576(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f577(const std::optional<T1> &a0)
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
T1 &f578(const std::optional<T1> &a0)
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
T1 &f579(const std::optional<T1> &a0)
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
bool f580(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f581(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f582(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f583(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f584(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f585(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f586(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f587(const std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f588(std::optional<T1> &a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f589(std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f590(const std::optional<T1> &&a0)
  requires((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f591(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f592(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f593(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f594(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f595(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f596(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f597(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f598(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f599(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f600(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f601(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f602(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f603(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f604(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f605(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f606(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f607(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f608(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f609(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f610(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f611(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f612(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f613(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f614(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f615(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f616(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f617(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f618(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f619(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f620(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f621(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f622(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f623(const std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f624(std::optional<T1> &a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f625(std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f626(const std::optional<T1> &&a0)
  requires(std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f627(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f628(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f629(std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
bool f630(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1> f631(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 &f632(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f633(const std::optional<T1> &a0)
  requires(std::is_reference<T1>::value == true) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
bool f634(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.has_value();
}

template <typename T1>
std::add_pointer_t<T1 const> f635(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
std::add_pointer_t<T1> f636(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator->();
}

template <typename T1>
T1 const &f637(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &f638(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.operator*();
}

template <typename T1>
T1 &&f639(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &&f640(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).operator*();
}

template <typename T1>
T1 const &f641(const std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &f642(std::optional<T1> &a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.value();
}

template <typename T1>
T1 &&f643(std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
T1 const &&f644(const std::optional<T1> &&a0)
  requires(std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return std::move(a0).value();
}

template <typename T1>
void f645(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f646(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
          (std::is_trivially_copy_constructible_v<T1> == false) &&
          (std::is_trivially_move_constructible_v<T1> == false) &&
          ((std::is_trivially_destructible_v<T1> &&
            std::is_trivially_move_constructible_v<T1> &&
            std::is_trivially_move_assignable_v<T1>) == false)
{
  return a0.reset();
}

template <typename T1>
void f647(std::optional<T1> &a0)
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
bool f648(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f649(const std::optional<T1> &a0)
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
T1 &f650(const std::optional<T1> &a0)
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
T1 &f651(const std::optional<T1> &a0)
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
bool f652(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f653(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f654(std::optional<T1> &a0)
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
T1 const &f655(const std::optional<T1> &a0)
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
T1 &f656(std::optional<T1> &a0)
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
T1 &&f657(std::optional<T1> &&a0)
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
T1 const &&f658(const std::optional<T1> &&a0)
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
T1 const &f659(const std::optional<T1> &a0)
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
T1 &f660(std::optional<T1> &a0)
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
T1 &&f661(std::optional<T1> &&a0)
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
T1 const &&f662(const std::optional<T1> &&a0)
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
void f663(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
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
void f664(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
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
void f665(std::optional<T1> &a0)
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
bool f666(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f667(const std::optional<T1> &a0)
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
T1 &f668(const std::optional<T1> &a0)
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
T1 &f669(const std::optional<T1> &a0)
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
bool f670(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f671(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f672(std::optional<T1> &a0)
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
T1 const &f673(const std::optional<T1> &a0)
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
T1 &f674(std::optional<T1> &a0)
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
T1 &&f675(std::optional<T1> &&a0)
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
T1 const &&f676(const std::optional<T1> &&a0)
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
T1 const &f677(const std::optional<T1> &a0)
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
T1 &f678(std::optional<T1> &a0)
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
T1 &&f679(std::optional<T1> &&a0)
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
T1 const &&f680(const std::optional<T1> &&a0)
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
void f681(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
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
void f682(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
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
void f683(std::optional<T1> &a0)
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
bool f684(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f685(const std::optional<T1> &a0)
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
T1 &f686(const std::optional<T1> &a0)
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
T1 &f687(const std::optional<T1> &a0)
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
bool f688(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f689(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f690(std::optional<T1> &a0)
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
T1 const &f691(const std::optional<T1> &a0)
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
T1 &f692(std::optional<T1> &a0)
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
T1 &&f693(std::optional<T1> &&a0)
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
T1 const &&f694(const std::optional<T1> &&a0)
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
T1 const &f695(const std::optional<T1> &a0)
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
T1 &f696(std::optional<T1> &a0)
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
T1 &&f697(std::optional<T1> &&a0)
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
T1 const &&f698(const std::optional<T1> &&a0)
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
void f699(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
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
void f700(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
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
void f701(std::optional<T1> &a0)
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
bool f702(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f703(const std::optional<T1> &a0)
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
T1 &f704(const std::optional<T1> &a0)
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
T1 &f705(const std::optional<T1> &a0)
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
bool f706(const std::optional<T1> &a0)
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
std::add_pointer_t<T1 const> f707(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f708(std::optional<T1> &a0)
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
T1 const &f709(const std::optional<T1> &a0)
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
T1 &f710(std::optional<T1> &a0)
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
T1 &&f711(std::optional<T1> &&a0)
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
T1 const &&f712(const std::optional<T1> &&a0)
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
T1 const &f713(const std::optional<T1> &a0)
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
T1 &f714(std::optional<T1> &a0)
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
T1 &&f715(std::optional<T1> &&a0)
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
T1 const &&f716(const std::optional<T1> &&a0)
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
void f717(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == false) &&
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
void f718(std::optional<T1> &a0)
  requires(std::is_trivially_destructible<T1>::value == true) &&
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
void f719(std::optional<T1> &a0)
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
bool f720(const std::optional<T1> &a0)
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
std::add_pointer_t<T1> f721(const std::optional<T1> &a0)
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
T1 &f722(const std::optional<T1> &a0)
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
T1 &f723(const std::optional<T1> &a0)
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
