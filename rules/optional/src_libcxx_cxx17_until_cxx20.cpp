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
std::optional<T1> f150(T2 &&a0) {
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
std::optional<T1> f151(T2 &&a0) {
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
std::optional<T1> f152(const std::optional<T2> &a0) {
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
std::optional<T1> f153(const std::optional<T2> &a0) {
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
std::optional<T1> f154(std::optional<T2> &&a0) {
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
std::optional<T1> f155(std::optional<T2> &&a0) {
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
std::optional<T1> &f156(std::optional<T1> &a0, T2 &&a1) {
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
std::optional<T1> &f157(std::optional<T1> &a0, const std::optional<T2> &a1) {
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
std::optional<T1> &f158(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
void f159(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
bool f160(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f161(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f162(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f163(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f164(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f165(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f166(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f167(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f168(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f169(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f170(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f171(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f172(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f173(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f174(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f175(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f176(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f177(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f178(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f179(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f180(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f181(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f182(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f183(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f184(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f185(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f186(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f187(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f188(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f189(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f190(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f191(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f192(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f193(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f194(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f195(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f196(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1 const> f197(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f198(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f199(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f200(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f201(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f202(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f203(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f204(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f205(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f206(const std::optional<T1> &&a0) {
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
void f207(std::optional<T1> &a0) {
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
bool f208(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f209(const std::optional<T1> &a0) {
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
T1 &f210(const std::optional<T1> &a0) {
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
T1 &f211(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f212(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f213(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f214(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f215(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f216(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f217(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f218(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f219(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f220(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f221(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f222(const std::optional<T1> &&a0) {
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
void f223(std::optional<T1> &a0) {
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
bool f224(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f225(const std::optional<T1> &a0) {
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
T1 &f226(const std::optional<T1> &a0) {
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
T1 &f227(const std::optional<T1> &a0) {
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
bool f228(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f229(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f230(std::optional<T1> &a0) {
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
T1 const &f231(const std::optional<T1> &a0) {
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
T1 &f232(std::optional<T1> &a0) {
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
T1 &&f233(std::optional<T1> &&a0) {
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
T1 const &&f234(const std::optional<T1> &&a0) {
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
T1 const &f235(const std::optional<T1> &a0) {
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
T1 &f236(std::optional<T1> &a0) {
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
T1 &&f237(std::optional<T1> &&a0) {
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
T1 const &&f238(const std::optional<T1> &&a0) {
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
void f239(std::optional<T1> &a0) {
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
bool f240(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f241(const std::optional<T1> &a0) {
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
T1 &f242(const std::optional<T1> &a0) {
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
T1 &f243(const std::optional<T1> &a0) {
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
bool f244(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f245(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f246(std::optional<T1> &a0) {
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
T1 const &f247(const std::optional<T1> &a0) {
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
T1 &f248(std::optional<T1> &a0) {
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
T1 &&f249(std::optional<T1> &&a0) {
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
T1 const &&f250(const std::optional<T1> &&a0) {
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
T1 const &f251(const std::optional<T1> &a0) {
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
T1 &f252(std::optional<T1> &a0) {
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
T1 &&f253(std::optional<T1> &&a0) {
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
T1 const &&f254(const std::optional<T1> &&a0) {
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
void f255(std::optional<T1> &a0) {
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
bool f256(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f257(const std::optional<T1> &a0) {
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
T1 &f258(const std::optional<T1> &a0) {
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
T1 &f259(const std::optional<T1> &a0) {
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
bool f260(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f261(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f262(std::optional<T1> &a0) {
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
T1 const &f263(const std::optional<T1> &a0) {
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
T1 &f264(std::optional<T1> &a0) {
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
T1 &&f265(std::optional<T1> &&a0) {
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
T1 const &&f266(const std::optional<T1> &&a0) {
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
T1 const &f267(const std::optional<T1> &a0) {
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
T1 &f268(std::optional<T1> &a0) {
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
T1 &&f269(std::optional<T1> &&a0) {
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
T1 const &&f270(const std::optional<T1> &&a0) {
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
void f271(std::optional<T1> &a0) {
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
bool f272(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f273(const std::optional<T1> &a0) {
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
T1 &f274(const std::optional<T1> &a0) {
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
T1 &f275(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f276(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f277(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f278(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f279(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f280(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f281(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f282(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f283(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f284(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f285(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f286(const std::optional<T1> &&a0) {
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
void f287(std::optional<T1> &a0) {
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
bool f288(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f289(const std::optional<T1> &a0) {
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
T1 &f290(const std::optional<T1> &a0) {
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
T1 &f291(const std::optional<T1> &a0) {
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
bool f292(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f293(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f294(std::optional<T1> &a0) {
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
T1 const &f295(const std::optional<T1> &a0) {
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
T1 &f296(std::optional<T1> &a0) {
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
T1 &&f297(std::optional<T1> &&a0) {
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
T1 const &&f298(const std::optional<T1> &&a0) {
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
T1 const &f299(const std::optional<T1> &a0) {
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
T1 &f300(std::optional<T1> &a0) {
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
T1 &&f301(std::optional<T1> &&a0) {
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
T1 const &&f302(const std::optional<T1> &&a0) {
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
void f303(std::optional<T1> &a0) {
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
bool f304(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f305(const std::optional<T1> &a0) {
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
T1 &f306(const std::optional<T1> &a0) {
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
T1 &f307(const std::optional<T1> &a0) {
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
bool f308(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f309(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f310(std::optional<T1> &a0) {
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
T1 const &f311(const std::optional<T1> &a0) {
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
T1 &f312(std::optional<T1> &a0) {
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
T1 &&f313(std::optional<T1> &&a0) {
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
T1 const &&f314(const std::optional<T1> &&a0) {
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
T1 const &f315(const std::optional<T1> &a0) {
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
T1 &f316(std::optional<T1> &a0) {
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
T1 &&f317(std::optional<T1> &&a0) {
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
T1 const &&f318(const std::optional<T1> &&a0) {
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
void f319(std::optional<T1> &a0) {
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
bool f320(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f321(const std::optional<T1> &a0) {
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
T1 &f322(const std::optional<T1> &a0) {
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
T1 &f323(const std::optional<T1> &a0) {
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
bool f324(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f325(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f326(std::optional<T1> &a0) {
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
T1 const &f327(const std::optional<T1> &a0) {
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
T1 &f328(std::optional<T1> &a0) {
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
T1 &&f329(std::optional<T1> &&a0) {
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
T1 const &&f330(const std::optional<T1> &&a0) {
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
T1 const &f331(const std::optional<T1> &a0) {
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
T1 &f332(std::optional<T1> &a0) {
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
T1 &&f333(std::optional<T1> &&a0) {
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
T1 const &&f334(const std::optional<T1> &&a0) {
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
void f335(std::optional<T1> &a0) {
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
bool f336(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f337(const std::optional<T1> &a0) {
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
T1 &f338(const std::optional<T1> &a0) {
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
T1 &f339(const std::optional<T1> &a0) {
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
bool f340(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f341(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f342(std::optional<T1> &a0) {
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
T1 const &f343(const std::optional<T1> &a0) {
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
T1 &f344(std::optional<T1> &a0) {
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
T1 &&f345(std::optional<T1> &&a0) {
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
T1 const &&f346(const std::optional<T1> &&a0) {
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
T1 const &f347(const std::optional<T1> &a0) {
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
T1 &f348(std::optional<T1> &a0) {
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
T1 &&f349(std::optional<T1> &&a0) {
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
T1 const &&f350(const std::optional<T1> &&a0) {
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
void f351(std::optional<T1> &a0) {
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
bool f352(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f353(const std::optional<T1> &a0) {
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
T1 &f354(const std::optional<T1> &a0) {
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
T1 &f355(const std::optional<T1> &a0) {
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
bool f356(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f357(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f358(std::optional<T1> &a0) {
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
T1 const &f359(const std::optional<T1> &a0) {
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
T1 &f360(std::optional<T1> &a0) {
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
T1 &&f361(std::optional<T1> &&a0) {
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
T1 const &&f362(const std::optional<T1> &&a0) {
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
T1 const &f363(const std::optional<T1> &a0) {
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
T1 &f364(std::optional<T1> &a0) {
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
T1 &&f365(std::optional<T1> &&a0) {
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
T1 const &&f366(const std::optional<T1> &&a0) {
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
void f367(std::optional<T1> &a0) {
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
bool f368(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f369(const std::optional<T1> &a0) {
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
T1 &f370(const std::optional<T1> &a0) {
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
T1 &f371(const std::optional<T1> &a0) {
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
bool f372(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f373(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f374(std::optional<T1> &a0) {
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
T1 const &f375(const std::optional<T1> &a0) {
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
T1 &f376(std::optional<T1> &a0) {
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
T1 &&f377(std::optional<T1> &&a0) {
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
T1 const &&f378(const std::optional<T1> &&a0) {
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
T1 const &f379(const std::optional<T1> &a0) {
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
T1 &f380(std::optional<T1> &a0) {
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
T1 &&f381(std::optional<T1> &&a0) {
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
T1 const &&f382(const std::optional<T1> &&a0) {
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
void f383(std::optional<T1> &a0) {
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
bool f384(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f385(const std::optional<T1> &a0) {
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
T1 &f386(const std::optional<T1> &a0) {
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
T1 &f387(const std::optional<T1> &a0) {
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
bool f388(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f389(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f390(std::optional<T1> &a0) {
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
T1 const &f391(const std::optional<T1> &a0) {
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
T1 &f392(std::optional<T1> &a0) {
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
T1 &&f393(std::optional<T1> &&a0) {
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
T1 const &&f394(const std::optional<T1> &&a0) {
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
T1 const &f395(const std::optional<T1> &a0) {
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
T1 &f396(std::optional<T1> &a0) {
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
T1 &&f397(std::optional<T1> &&a0) {
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
T1 const &&f398(const std::optional<T1> &&a0) {
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
void f399(std::optional<T1> &a0) {
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
bool f400(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f401(const std::optional<T1> &a0) {
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
T1 &f402(const std::optional<T1> &a0) {
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
T1 &f403(const std::optional<T1> &a0) {
  return a0.value();
}
