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
std::optional<T1> f116(T2 &&a0) {
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
std::optional<T1> f117(T2 &&a0) {
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
std::optional<T1> f118(const std::optional<T2> &a0) {
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
std::optional<T1> f119(const std::optional<T2> &a0) {
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
std::optional<T1> f120(std::optional<T2> &&a0) {
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
std::optional<T1> f121(std::optional<T2> &&a0) {
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
std::optional<T1> &f122(std::optional<T1> &a0, T2 &&a1) {
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
std::optional<T1> &f123(std::optional<T1> &a0, const std::optional<T2> &a1) {
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
std::optional<T1> &f124(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
void f125(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
bool f126(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f127(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f128(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f129(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f130(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f131(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f132(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f133(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f134(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f135(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f136(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f137(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f138(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f139(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f140(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f141(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f142(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f143(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f144(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f145(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f146(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f147(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f148(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f149(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f150(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f151(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f152(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f153(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f154(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f155(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f156(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f157(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f158(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f159(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f160(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f161(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f162(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1 const> f163(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f164(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f165(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f166(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f167(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f168(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f169(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f170(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f171(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f172(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f173(std::optional<T1> &a0) {
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
bool f174(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f175(const std::optional<T1> &a0) {
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
T1 &f176(const std::optional<T1> &a0) {
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
T1 &f177(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f178(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f179(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f180(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f181(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f182(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f183(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f184(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f185(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f186(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f187(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f188(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_copy_constructible_v<T1> &&
                std::is_trivially_copy_assignable_v<T1>) == false),
              int>::type T3 = 0>
void f189(std::optional<T1> &a0) {
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
bool f190(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f191(const std::optional<T1> &a0) {
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
T1 &f192(const std::optional<T1> &a0) {
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
T1 &f193(const std::optional<T1> &a0) {
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
bool f194(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f195(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f196(std::optional<T1> &a0) {
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
T1 const &f197(const std::optional<T1> &a0) {
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
T1 &f198(std::optional<T1> &a0) {
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
T1 &&f199(std::optional<T1> &&a0) {
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
T1 const &&f200(const std::optional<T1> &&a0) {
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
T1 const &f201(const std::optional<T1> &a0) {
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
T1 &f202(std::optional<T1> &a0) {
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
T1 &&f203(std::optional<T1> &&a0) {
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
T1 const &&f204(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f205(std::optional<T1> &a0) {
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
bool f206(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f207(const std::optional<T1> &a0) {
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
T1 &f208(const std::optional<T1> &a0) {
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
T1 &f209(const std::optional<T1> &a0) {
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
bool f210(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f211(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f212(std::optional<T1> &a0) {
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
T1 const &f213(const std::optional<T1> &a0) {
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
T1 &f214(std::optional<T1> &a0) {
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
T1 &&f215(std::optional<T1> &&a0) {
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
T1 const &&f216(const std::optional<T1> &&a0) {
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
T1 const &f217(const std::optional<T1> &a0) {
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
T1 &f218(std::optional<T1> &a0) {
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
T1 &&f219(std::optional<T1> &&a0) {
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
T1 const &&f220(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f221(std::optional<T1> &a0) {
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
bool f222(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f223(const std::optional<T1> &a0) {
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
T1 &f224(const std::optional<T1> &a0) {
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
T1 &f225(const std::optional<T1> &a0) {
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
bool f226(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f227(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f228(std::optional<T1> &a0) {
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
T1 const &f229(const std::optional<T1> &a0) {
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
T1 &f230(std::optional<T1> &a0) {
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
T1 &&f231(std::optional<T1> &&a0) {
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
T1 const &&f232(const std::optional<T1> &&a0) {
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
T1 const &f233(const std::optional<T1> &a0) {
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
T1 &f234(std::optional<T1> &a0) {
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
T1 &&f235(std::optional<T1> &&a0) {
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
T1 const &&f236(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f237(std::optional<T1> &a0) {
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
bool f238(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f239(const std::optional<T1> &a0) {
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
T1 &f240(const std::optional<T1> &a0) {
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
T1 &f241(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f242(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f243(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f244(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f245(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f246(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f247(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f248(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f249(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f250(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f251(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f252(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<
              ((std::is_trivially_destructible_v<T1> &&
                std::is_trivially_move_constructible_v<T1> &&
                std::is_trivially_move_assignable_v<T1>) == false),
              int>::type T3 = 0>
void f253(std::optional<T1> &a0) {
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
bool f254(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f255(const std::optional<T1> &a0) {
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
T1 &f256(const std::optional<T1> &a0) {
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
T1 &f257(const std::optional<T1> &a0) {
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
bool f258(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f259(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f260(std::optional<T1> &a0) {
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
T1 const &f261(const std::optional<T1> &a0) {
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
T1 &f262(std::optional<T1> &a0) {
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
T1 &&f263(std::optional<T1> &&a0) {
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
T1 const &&f264(const std::optional<T1> &&a0) {
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
T1 const &f265(const std::optional<T1> &a0) {
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
T1 &f266(std::optional<T1> &a0) {
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
T1 &&f267(std::optional<T1> &&a0) {
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
T1 const &&f268(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f269(std::optional<T1> &a0) {
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
bool f270(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f271(const std::optional<T1> &a0) {
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
T1 &f272(const std::optional<T1> &a0) {
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
T1 &f273(const std::optional<T1> &a0) {
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
bool f274(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f275(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f276(std::optional<T1> &a0) {
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
T1 const &f277(const std::optional<T1> &a0) {
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
T1 &f278(std::optional<T1> &a0) {
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
T1 &&f279(std::optional<T1> &&a0) {
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
T1 const &&f280(const std::optional<T1> &&a0) {
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
T1 const &f281(const std::optional<T1> &a0) {
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
T1 &f282(std::optional<T1> &a0) {
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
T1 &&f283(std::optional<T1> &&a0) {
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
T1 const &&f284(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f285(std::optional<T1> &a0) {
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
bool f286(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f287(const std::optional<T1> &a0) {
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
T1 &f288(const std::optional<T1> &a0) {
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
T1 &f289(const std::optional<T1> &a0) {
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
bool f290(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f291(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f292(std::optional<T1> &a0) {
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
T1 const &f293(const std::optional<T1> &a0) {
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
T1 &f294(std::optional<T1> &a0) {
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
T1 &&f295(std::optional<T1> &&a0) {
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
T1 const &&f296(const std::optional<T1> &&a0) {
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
T1 const &f297(const std::optional<T1> &a0) {
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
T1 &f298(std::optional<T1> &a0) {
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
T1 &&f299(std::optional<T1> &&a0) {
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
T1 const &&f300(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f301(std::optional<T1> &a0) {
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
bool f302(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f303(const std::optional<T1> &a0) {
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
T1 &f304(const std::optional<T1> &a0) {
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
T1 &f305(const std::optional<T1> &a0) {
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
bool f306(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f307(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f308(std::optional<T1> &a0) {
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
T1 const &f309(const std::optional<T1> &a0) {
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
T1 &f310(std::optional<T1> &a0) {
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
T1 &&f311(std::optional<T1> &&a0) {
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
T1 const &&f312(const std::optional<T1> &&a0) {
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
T1 const &f313(const std::optional<T1> &a0) {
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
T1 &f314(std::optional<T1> &a0) {
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
T1 &&f315(std::optional<T1> &&a0) {
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
T1 const &&f316(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f317(std::optional<T1> &a0) {
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
bool f318(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f319(const std::optional<T1> &a0) {
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
T1 &f320(const std::optional<T1> &a0) {
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
T1 &f321(const std::optional<T1> &a0) {
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
bool f322(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f323(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f324(std::optional<T1> &a0) {
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
T1 const &f325(const std::optional<T1> &a0) {
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
T1 &f326(std::optional<T1> &a0) {
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
T1 &&f327(std::optional<T1> &&a0) {
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
T1 const &&f328(const std::optional<T1> &&a0) {
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
T1 const &f329(const std::optional<T1> &a0) {
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
T1 &f330(std::optional<T1> &a0) {
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
T1 &&f331(std::optional<T1> &&a0) {
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
T1 const &&f332(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f333(std::optional<T1> &a0) {
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
bool f334(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f335(const std::optional<T1> &a0) {
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
T1 &f336(const std::optional<T1> &a0) {
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
T1 &f337(const std::optional<T1> &a0) {
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
bool f338(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f339(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f340(std::optional<T1> &a0) {
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
T1 const &f341(const std::optional<T1> &a0) {
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
T1 &f342(std::optional<T1> &a0) {
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
T1 &&f343(std::optional<T1> &&a0) {
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
T1 const &&f344(const std::optional<T1> &&a0) {
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
T1 const &f345(const std::optional<T1> &a0) {
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
T1 &f346(std::optional<T1> &a0) {
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
T1 &&f347(std::optional<T1> &&a0) {
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
T1 const &&f348(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f349(std::optional<T1> &a0) {
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
bool f350(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f351(const std::optional<T1> &a0) {
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
T1 &f352(const std::optional<T1> &a0) {
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
T1 &f353(const std::optional<T1> &a0) {
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
bool f354(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f355(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f356(std::optional<T1> &a0) {
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
T1 const &f357(const std::optional<T1> &a0) {
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
T1 &f358(std::optional<T1> &a0) {
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
T1 &&f359(std::optional<T1> &&a0) {
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
T1 const &&f360(const std::optional<T1> &&a0) {
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
T1 const &f361(const std::optional<T1> &a0) {
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
T1 &f362(std::optional<T1> &a0) {
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
T1 &&f363(std::optional<T1> &&a0) {
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
T1 const &&f364(const std::optional<T1> &&a0) {
  return std::move(a0).value();
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
void f365(std::optional<T1> &a0) {
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
bool f366(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f367(const std::optional<T1> &a0) {
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
T1 &f368(const std::optional<T1> &a0) {
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
T1 &f369(const std::optional<T1> &a0) {
  return a0.value();
}
