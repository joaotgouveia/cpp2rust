#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

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
        T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, const std::optional<T1> &>::value) &&
         !(std::is_same<T2, std::optional<T1>>::value) &&
         !(std::is_same<T2, std::nullopt_t>::value)),
        int>::type T4 = 0>
std::optional<T1> f151(T2 &&a0) {
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
        T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, const std::optional<T1> &>::value) &&
         !(std::is_same<T2, std::optional<T1>>::value) &&
         !(std::is_same<T2, std::nullopt_t>::value)),
        int>::type T4 = 0>
std::optional<T1> f152(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, const T2 &>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<const T2 &>,
                 std::__check_tuple_constructor_fail>::
            template __enable_implicit<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> f153(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, const T2 &>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<const T2 &>,
                 std::__check_tuple_constructor_fail>::
            template __enable_explicit<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> f154(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, T2 &&>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                 std::__check_tuple_constructor_fail>::
            template __enable_implicit<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> f155(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, T2 &&>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                 std::__check_tuple_constructor_fail>::
            template __enable_explicit<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> f156(std::optional<T2> &&a0) {
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
        T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, std::nullopt_t>::value) &&
         !(std::is_same<T2, const std::optional<T1> &>::value) &&
         !(std::is_same<T2, std::optional<T1>>::value)),
        int>::type T4 = 0>
std::optional<T1> &f157(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, const T2 &>,
                           std::is_assignable<T1 &, const T2 &>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<const T2 &>,
                 std::__check_tuple_constructor_fail>::
            template __enable_assign<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> &f158(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::enable_if_t<
        std::_If<std::_And<std::_IsNotSame<T2, T1>,
                           std::is_constructible<T1, T2 &&>>::value,
                 typename std::optional<
                     T1>::template _CheckOptionalLikeConstructor<T2 &&>,
                 std::__check_tuple_constructor_fail>::
            template __enable_assign<T2>(),
        int>
        T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::optional<T1> &f159(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename std::enable_if<
                           (std::is_trivially_destructible<T1>::value == false),
                           int>::type T2 = 0>
void f160(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1, typename std::enable_if<
                           (std::is_trivially_destructible<T1>::value == true),
                           int>::type T2 = 0>
void f161(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
void f162(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
bool f163(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f164(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f165(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f166(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f167(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f168(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f169(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f170(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f171(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f172(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f173(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f174(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f175(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f176(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f177(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0>
void f178(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0>
void f179(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f180(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f181(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f182(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f183(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f184(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f185(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f186(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f187(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f188(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f189(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f190(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f191(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f192(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f193(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f194(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f195(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0>
void f196(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0>
void f197(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f198(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f199(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f200(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f201(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f202(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f203(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1 const> f204(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f205(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f206(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f207(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f208(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f209(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f210(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f211(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f212(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f213(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0>
void f214(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0>
void f215(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0>
void f216(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0>
bool f217(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0>
std::add_pointer_t<T1> f218(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0>
T1 &f219(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0>
T1 &f220(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f221(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f222(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f223(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f224(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f225(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f226(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f227(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f228(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f229(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f230(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f231(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0>
void f232(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0>
void f233(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
void f234(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f235(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f236(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f237(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f238(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f239(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1 const> f240(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f241(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f242(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f243(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f244(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f245(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f246(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f247(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f248(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f249(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f250(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f251(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
void f252(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f253(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f254(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f255(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f256(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f257(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1 const> f258(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f259(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f260(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f261(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f262(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f263(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f264(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f265(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f266(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f267(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f268(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f269(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
void f270(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f271(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f272(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f273(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f274(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f275(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1 const> f276(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f277(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f278(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f279(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f280(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f281(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f282(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f283(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f284(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f285(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f286(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f287(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0>
void f288(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0>
bool f289(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1> f290(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f291(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f292(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f293(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f294(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f295(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f296(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f297(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f298(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f299(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f300(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f301(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f302(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f303(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0>
void f304(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0>
void f305(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
void f306(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f307(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f308(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f309(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f310(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f311(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1 const> f312(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f313(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f314(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f315(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f316(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f317(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f318(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f319(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f320(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f321(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f322(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f323(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
void f324(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f325(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f326(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f327(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f328(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f329(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1 const> f330(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f331(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f332(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f333(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f334(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f335(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f336(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f337(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f338(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f339(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f340(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f341(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
void f342(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f343(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f344(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f345(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f346(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f347(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1 const> f348(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f349(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f350(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f351(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f352(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f353(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f354(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f355(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f356(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f357(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f358(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f359(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
void f360(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
bool f361(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1> f362(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f363(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f364(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
bool f365(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1 const> f366(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
std::add_pointer_t<T1> f367(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f368(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f369(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f370(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f371(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &f372(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &f373(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 &&f374(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
T1 const &&f375(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f376(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T3 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0>
void f377(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
void f378(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f379(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f380(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f381(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f382(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f383(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1 const> f384(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f385(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f386(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f387(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f388(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f389(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f390(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f391(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f392(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f393(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f394(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f395(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
void f396(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
bool f397(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1> f398(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f399(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f400(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
bool f401(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1 const> f402(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
std::add_pointer_t<T1> f403(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f404(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f405(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f406(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f407(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &f408(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &f409(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 &&f410(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T4 = 0>
T1 const &&f411(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f412(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T4 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0>
void f413(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
void f414(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
bool f415(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1> f416(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f417(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f418(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
bool f419(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1 const> f420(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
std::add_pointer_t<T1> f421(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 const &f422(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f423(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &&f424(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 const &&f425(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 const &f426(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &f427(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 &&f428(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T5 = 0>
T1 const &&f429(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <
    typename T1,
    typename std::enable_if<
        (std::is_trivially_destructible<T1>::value == false), int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T6 = 0>
void f430(std::optional<T1> &a0) {
  return a0.reset();
}

template <
    typename T1,
    typename std::enable_if<(std::is_trivially_destructible<T1>::value == true),
                            int>::type T2 = 0,
    typename std::enable_if<
        (std::is_trivially_copy_constructible_v<T1> == false), int>::type T3 =
        0,
    typename std::enable_if<
        (std::is_trivially_move_constructible_v<T1> == false), int>::type T4 =
        0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_copy_constructible_v<T1> &&
                              std::is_trivially_copy_assignable_v<T1>) ==
                             false),
                            int>::type T5 = 0,
    typename std::enable_if<((std::is_trivially_destructible_v<T1> &&
                              std::is_trivially_move_constructible_v<T1> &&
                              std::is_trivially_move_assignable_v<T1>) ==
                             false),
                            int>::type T6 = 0>
void f431(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T6 = 0>
void f432(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T6 = 0>
bool f433(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T6 = 0>
std::add_pointer_t<T1> f434(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T6 = 0>
T1 &f435(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T4 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T5 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T6 = 0>
T1 &f436(const std::optional<T1> &a0) {
  return a0.value();
}
