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
std::optional<T1> f119(T2 &&a0) {
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
std::optional<T1> f120(T2 &&a0) {
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
std::optional<T1> f121(const std::optional<T2> &a0) {
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
std::optional<T1> f122(const std::optional<T2> &a0) {
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
std::optional<T1> f123(std::optional<T2> &&a0) {
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
std::optional<T1> f124(std::optional<T2> &&a0) {
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
std::optional<T1> &f125(std::optional<T1> &a0, T2 &&a1) {
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
std::optional<T1> &f126(std::optional<T1> &a0, const std::optional<T2> &a1) {
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
std::optional<T1> &f127(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
void f128(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
bool f129(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f130(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f131(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f132(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f133(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f134(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f135(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f136(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f137(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f138(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f139(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f140(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f141(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f142(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f143(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f144(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f145(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f146(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f147(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f148(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f149(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f150(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f151(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f152(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f153(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f154(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f155(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f156(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f157(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f158(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f159(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f160(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f161(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f162(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f163(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f164(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f165(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1 const> f166(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f167(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f168(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f169(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f170(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f171(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f172(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f173(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f174(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f175(const std::optional<T1> &&a0) {
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
void f176(std::optional<T1> &a0) {
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
bool f177(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f178(const std::optional<T1> &a0) {
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
T1 &f179(const std::optional<T1> &a0) {
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
T1 &f180(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f181(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f182(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f183(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f184(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f185(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f186(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f187(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f188(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f189(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f190(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f191(const std::optional<T1> &&a0) {
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
void f192(std::optional<T1> &a0) {
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
bool f193(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f194(const std::optional<T1> &a0) {
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
T1 &f195(const std::optional<T1> &a0) {
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
T1 &f196(const std::optional<T1> &a0) {
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
bool f197(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f198(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f199(std::optional<T1> &a0) {
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
T1 const &f200(const std::optional<T1> &a0) {
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
T1 &f201(std::optional<T1> &a0) {
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
T1 &&f202(std::optional<T1> &&a0) {
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
T1 const &&f203(const std::optional<T1> &&a0) {
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
T1 const &f204(const std::optional<T1> &a0) {
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
T1 &f205(std::optional<T1> &a0) {
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
T1 &&f206(std::optional<T1> &&a0) {
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
T1 const &&f207(const std::optional<T1> &&a0) {
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
void f208(std::optional<T1> &a0) {
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
bool f209(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f210(const std::optional<T1> &a0) {
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
T1 &f211(const std::optional<T1> &a0) {
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
T1 &f212(const std::optional<T1> &a0) {
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
bool f213(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f214(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f215(std::optional<T1> &a0) {
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
T1 const &f216(const std::optional<T1> &a0) {
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
T1 &f217(std::optional<T1> &a0) {
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
T1 &&f218(std::optional<T1> &&a0) {
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
T1 const &&f219(const std::optional<T1> &&a0) {
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
T1 const &f220(const std::optional<T1> &a0) {
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
T1 &f221(std::optional<T1> &a0) {
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
T1 &&f222(std::optional<T1> &&a0) {
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
T1 const &&f223(const std::optional<T1> &&a0) {
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
void f224(std::optional<T1> &a0) {
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
bool f225(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f226(const std::optional<T1> &a0) {
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
T1 &f227(const std::optional<T1> &a0) {
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
T1 &f228(const std::optional<T1> &a0) {
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
bool f229(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f230(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f231(std::optional<T1> &a0) {
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
T1 const &f232(const std::optional<T1> &a0) {
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
T1 &f233(std::optional<T1> &a0) {
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
T1 &&f234(std::optional<T1> &&a0) {
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
T1 const &&f235(const std::optional<T1> &&a0) {
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
T1 const &f236(const std::optional<T1> &a0) {
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
T1 &f237(std::optional<T1> &a0) {
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
T1 &&f238(std::optional<T1> &&a0) {
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
T1 const &&f239(const std::optional<T1> &&a0) {
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
void f240(std::optional<T1> &a0) {
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
bool f241(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f242(const std::optional<T1> &a0) {
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
T1 &f243(const std::optional<T1> &a0) {
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
T1 &f244(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f245(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f246(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f247(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f248(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f249(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f250(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f251(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f252(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f253(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f254(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f255(const std::optional<T1> &&a0) {
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
void f256(std::optional<T1> &a0) {
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
bool f257(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f258(const std::optional<T1> &a0) {
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
T1 &f259(const std::optional<T1> &a0) {
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
T1 &f260(const std::optional<T1> &a0) {
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
bool f261(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f262(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f263(std::optional<T1> &a0) {
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
T1 const &f264(const std::optional<T1> &a0) {
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
T1 &f265(std::optional<T1> &a0) {
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
T1 &&f266(std::optional<T1> &&a0) {
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
T1 const &&f267(const std::optional<T1> &&a0) {
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
T1 const &f268(const std::optional<T1> &a0) {
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
T1 &f269(std::optional<T1> &a0) {
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
T1 &&f270(std::optional<T1> &&a0) {
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
T1 const &&f271(const std::optional<T1> &&a0) {
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
void f272(std::optional<T1> &a0) {
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
bool f273(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f274(const std::optional<T1> &a0) {
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
T1 &f275(const std::optional<T1> &a0) {
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
T1 &f276(const std::optional<T1> &a0) {
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
bool f277(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f278(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f279(std::optional<T1> &a0) {
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
T1 const &f280(const std::optional<T1> &a0) {
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
T1 &f281(std::optional<T1> &a0) {
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
T1 &&f282(std::optional<T1> &&a0) {
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
T1 const &&f283(const std::optional<T1> &&a0) {
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
T1 const &f284(const std::optional<T1> &a0) {
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
T1 &f285(std::optional<T1> &a0) {
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
T1 &&f286(std::optional<T1> &&a0) {
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
T1 const &&f287(const std::optional<T1> &&a0) {
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
void f288(std::optional<T1> &a0) {
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
bool f289(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f290(const std::optional<T1> &a0) {
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
T1 &f291(const std::optional<T1> &a0) {
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
T1 &f292(const std::optional<T1> &a0) {
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
bool f293(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f294(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f295(std::optional<T1> &a0) {
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
T1 const &f296(const std::optional<T1> &a0) {
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
T1 &f297(std::optional<T1> &a0) {
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
T1 &&f298(std::optional<T1> &&a0) {
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
T1 const &&f299(const std::optional<T1> &&a0) {
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
T1 const &f300(const std::optional<T1> &a0) {
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
T1 &f301(std::optional<T1> &a0) {
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
T1 &&f302(std::optional<T1> &&a0) {
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
T1 const &&f303(const std::optional<T1> &&a0) {
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
void f304(std::optional<T1> &a0) {
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
bool f305(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f306(const std::optional<T1> &a0) {
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
T1 &f307(const std::optional<T1> &a0) {
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
T1 &f308(const std::optional<T1> &a0) {
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
bool f309(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f310(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f311(std::optional<T1> &a0) {
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
T1 const &f312(const std::optional<T1> &a0) {
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
T1 &f313(std::optional<T1> &a0) {
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
T1 &&f314(std::optional<T1> &&a0) {
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
T1 const &&f315(const std::optional<T1> &&a0) {
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
T1 const &f316(const std::optional<T1> &a0) {
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
T1 &f317(std::optional<T1> &a0) {
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
T1 &&f318(std::optional<T1> &&a0) {
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
T1 const &&f319(const std::optional<T1> &&a0) {
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
void f320(std::optional<T1> &a0) {
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
bool f321(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f322(const std::optional<T1> &a0) {
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
T1 &f323(const std::optional<T1> &a0) {
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
T1 &f324(const std::optional<T1> &a0) {
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
bool f325(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f326(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f327(std::optional<T1> &a0) {
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
T1 const &f328(const std::optional<T1> &a0) {
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
T1 &f329(std::optional<T1> &a0) {
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
T1 &&f330(std::optional<T1> &&a0) {
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
T1 const &&f331(const std::optional<T1> &&a0) {
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
T1 const &f332(const std::optional<T1> &a0) {
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
T1 &f333(std::optional<T1> &a0) {
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
T1 &&f334(std::optional<T1> &&a0) {
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
T1 const &&f335(const std::optional<T1> &&a0) {
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
void f336(std::optional<T1> &a0) {
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
bool f337(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f338(const std::optional<T1> &a0) {
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
T1 &f339(const std::optional<T1> &a0) {
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
T1 &f340(const std::optional<T1> &a0) {
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
bool f341(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f342(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f343(std::optional<T1> &a0) {
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
T1 const &f344(const std::optional<T1> &a0) {
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
T1 &f345(std::optional<T1> &a0) {
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
T1 &&f346(std::optional<T1> &&a0) {
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
T1 const &&f347(const std::optional<T1> &&a0) {
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
T1 const &f348(const std::optional<T1> &a0) {
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
T1 &f349(std::optional<T1> &a0) {
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
T1 &&f350(std::optional<T1> &&a0) {
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
T1 const &&f351(const std::optional<T1> &&a0) {
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
void f352(std::optional<T1> &a0) {
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
bool f353(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f354(const std::optional<T1> &a0) {
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
T1 &f355(const std::optional<T1> &a0) {
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
T1 &f356(const std::optional<T1> &a0) {
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
bool f357(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f358(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f359(std::optional<T1> &a0) {
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
T1 const &f360(const std::optional<T1> &a0) {
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
T1 &f361(std::optional<T1> &a0) {
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
T1 &&f362(std::optional<T1> &&a0) {
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
T1 const &&f363(const std::optional<T1> &&a0) {
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
T1 const &f364(const std::optional<T1> &a0) {
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
T1 &f365(std::optional<T1> &a0) {
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
T1 &&f366(std::optional<T1> &&a0) {
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
T1 const &&f367(const std::optional<T1> &&a0) {
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
void f368(std::optional<T1> &a0) {
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
bool f369(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f370(const std::optional<T1> &a0) {
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
T1 &f371(const std::optional<T1> &a0) {
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
T1 &f372(const std::optional<T1> &a0) {
  return a0.value();
}
