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
std::optional<T1> f149(T2 &&a0) {
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
std::optional<T1> f150(T2 &&a0) {
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
std::optional<T1> f151(const std::optional<T2> &a0) {
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
std::optional<T1> f152(const std::optional<T2> &a0) {
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
std::optional<T1> f153(std::optional<T2> &&a0) {
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
std::optional<T1> f154(std::optional<T2> &&a0) {
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
std::optional<T1> &f155(std::optional<T1> &a0, T2 &&a1) {
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
std::optional<T1> &f156(std::optional<T1> &a0, const std::optional<T2> &a1) {
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
std::optional<T1> &f157(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
void f158(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
bool f159(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f160(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f161(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0>
T1 &f162(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f163(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f164(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f165(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f166(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f167(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f168(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f169(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f170(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f171(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f172(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f173(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f174(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f175(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f176(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f177(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f178(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
bool f179(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1 const> f180(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
std::add_pointer_t<T1> f181(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f182(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f183(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f184(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f185(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &f186(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &f187(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 &&f188(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0>
T1 const &&f189(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
void f190(std::optional<T1> &a0) {
  return a0.reset();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f191(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f192(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f193(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_reference<T1>::value == true),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f194(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
bool f195(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1 const> f196(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
std::add_pointer_t<T1> f197(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f198(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f199(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f200(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f201(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &f202(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &f203(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 &&f204(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1,
          typename std::enable_if<(std::is_trivially_copy_constructible_v<T1> ==
                                   false),
                                  int>::type T2 = 0,
          typename std::enable_if<(std::is_trivially_move_constructible_v<T1> ==
                                   false),
                                  int>::type T3 = 0>
T1 const &&f205(const std::optional<T1> &&a0) {
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
void f206(std::optional<T1> &a0) {
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
bool f207(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f208(const std::optional<T1> &a0) {
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
T1 &f209(const std::optional<T1> &a0) {
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
T1 &f210(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f211(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f212(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f213(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f214(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f215(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f216(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f217(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f218(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f219(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f220(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_copy_constructible_v<T1> &&
                             std::is_trivially_copy_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f221(const std::optional<T1> &&a0) {
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
void f222(std::optional<T1> &a0) {
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
bool f223(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f224(const std::optional<T1> &a0) {
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
T1 &f225(const std::optional<T1> &a0) {
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
T1 &f226(const std::optional<T1> &a0) {
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
bool f227(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f228(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f229(std::optional<T1> &a0) {
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
T1 const &f230(const std::optional<T1> &a0) {
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
T1 &f231(std::optional<T1> &a0) {
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
T1 &&f232(std::optional<T1> &&a0) {
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
T1 const &&f233(const std::optional<T1> &&a0) {
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
T1 const &f234(const std::optional<T1> &a0) {
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
T1 &f235(std::optional<T1> &a0) {
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
T1 &&f236(std::optional<T1> &&a0) {
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
T1 const &&f237(const std::optional<T1> &&a0) {
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
void f238(std::optional<T1> &a0) {
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
bool f239(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f240(const std::optional<T1> &a0) {
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
T1 &f241(const std::optional<T1> &a0) {
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
T1 &f242(const std::optional<T1> &a0) {
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
bool f243(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f244(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f245(std::optional<T1> &a0) {
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
T1 const &f246(const std::optional<T1> &a0) {
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
T1 &f247(std::optional<T1> &a0) {
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
T1 &&f248(std::optional<T1> &&a0) {
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
T1 const &&f249(const std::optional<T1> &&a0) {
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
T1 const &f250(const std::optional<T1> &a0) {
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
T1 &f251(std::optional<T1> &a0) {
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
T1 &&f252(std::optional<T1> &&a0) {
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
T1 const &&f253(const std::optional<T1> &&a0) {
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
void f254(std::optional<T1> &a0) {
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
bool f255(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f256(const std::optional<T1> &a0) {
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
T1 &f257(const std::optional<T1> &a0) {
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
T1 &f258(const std::optional<T1> &a0) {
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
bool f259(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f260(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f261(std::optional<T1> &a0) {
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
T1 const &f262(const std::optional<T1> &a0) {
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
T1 &f263(std::optional<T1> &a0) {
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
T1 &&f264(std::optional<T1> &&a0) {
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
T1 const &&f265(const std::optional<T1> &&a0) {
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
T1 const &f266(const std::optional<T1> &a0) {
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
T1 &f267(std::optional<T1> &a0) {
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
T1 &&f268(std::optional<T1> &&a0) {
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
T1 const &&f269(const std::optional<T1> &&a0) {
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
void f270(std::optional<T1> &a0) {
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
bool f271(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f272(const std::optional<T1> &a0) {
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
T1 &f273(const std::optional<T1> &a0) {
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
T1 &f274(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
bool f275(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1 const> f276(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
std::add_pointer_t<T1> f277(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f278(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f279(std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f280(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f281(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &f282(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &f283(std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 &&f284(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename std::enable_if<
                           ((std::is_trivially_destructible_v<T1> &&
                             std::is_trivially_move_constructible_v<T1> &&
                             std::is_trivially_move_assignable_v<T1>) == false),
                           int>::type T2 = 0>
T1 const &&f285(const std::optional<T1> &&a0) {
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
void f286(std::optional<T1> &a0) {
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
bool f287(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f288(const std::optional<T1> &a0) {
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
T1 &f289(const std::optional<T1> &a0) {
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
T1 &f290(const std::optional<T1> &a0) {
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
bool f291(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f292(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f293(std::optional<T1> &a0) {
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
T1 const &f294(const std::optional<T1> &a0) {
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
T1 &f295(std::optional<T1> &a0) {
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
T1 &&f296(std::optional<T1> &&a0) {
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
T1 const &&f297(const std::optional<T1> &&a0) {
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
T1 const &f298(const std::optional<T1> &a0) {
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
T1 &f299(std::optional<T1> &a0) {
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
T1 &&f300(std::optional<T1> &&a0) {
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
T1 const &&f301(const std::optional<T1> &&a0) {
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
void f302(std::optional<T1> &a0) {
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
bool f303(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f304(const std::optional<T1> &a0) {
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
T1 &f305(const std::optional<T1> &a0) {
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
T1 &f306(const std::optional<T1> &a0) {
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
bool f307(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f308(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f309(std::optional<T1> &a0) {
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
T1 const &f310(const std::optional<T1> &a0) {
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
T1 &f311(std::optional<T1> &a0) {
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
T1 &&f312(std::optional<T1> &&a0) {
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
T1 const &&f313(const std::optional<T1> &&a0) {
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
T1 const &f314(const std::optional<T1> &a0) {
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
T1 &f315(std::optional<T1> &a0) {
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
T1 &&f316(std::optional<T1> &&a0) {
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
T1 const &&f317(const std::optional<T1> &&a0) {
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
void f318(std::optional<T1> &a0) {
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
bool f319(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f320(const std::optional<T1> &a0) {
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
T1 &f321(const std::optional<T1> &a0) {
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
T1 &f322(const std::optional<T1> &a0) {
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
bool f323(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f324(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f325(std::optional<T1> &a0) {
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
T1 const &f326(const std::optional<T1> &a0) {
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
T1 &f327(std::optional<T1> &a0) {
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
T1 &&f328(std::optional<T1> &&a0) {
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
T1 const &&f329(const std::optional<T1> &&a0) {
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
T1 const &f330(const std::optional<T1> &a0) {
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
T1 &f331(std::optional<T1> &a0) {
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
T1 &&f332(std::optional<T1> &&a0) {
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
T1 const &&f333(const std::optional<T1> &&a0) {
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
void f334(std::optional<T1> &a0) {
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
bool f335(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f336(const std::optional<T1> &a0) {
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
T1 &f337(const std::optional<T1> &a0) {
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
T1 &f338(const std::optional<T1> &a0) {
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
bool f339(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f340(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f341(std::optional<T1> &a0) {
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
T1 const &f342(const std::optional<T1> &a0) {
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
T1 &f343(std::optional<T1> &a0) {
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
T1 &&f344(std::optional<T1> &&a0) {
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
T1 const &&f345(const std::optional<T1> &&a0) {
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
T1 const &f346(const std::optional<T1> &a0) {
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
T1 &f347(std::optional<T1> &a0) {
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
T1 &&f348(std::optional<T1> &&a0) {
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
T1 const &&f349(const std::optional<T1> &&a0) {
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
void f350(std::optional<T1> &a0) {
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
bool f351(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f352(const std::optional<T1> &a0) {
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
T1 &f353(const std::optional<T1> &a0) {
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
T1 &f354(const std::optional<T1> &a0) {
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
bool f355(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f356(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f357(std::optional<T1> &a0) {
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
T1 const &f358(const std::optional<T1> &a0) {
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
T1 &f359(std::optional<T1> &a0) {
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
T1 &&f360(std::optional<T1> &&a0) {
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
T1 const &&f361(const std::optional<T1> &&a0) {
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
T1 const &f362(const std::optional<T1> &a0) {
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
T1 &f363(std::optional<T1> &a0) {
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
T1 &&f364(std::optional<T1> &&a0) {
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
T1 const &&f365(const std::optional<T1> &&a0) {
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
void f366(std::optional<T1> &a0) {
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
bool f367(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f368(const std::optional<T1> &a0) {
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
T1 &f369(const std::optional<T1> &a0) {
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
T1 &f370(const std::optional<T1> &a0) {
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
bool f371(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f372(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f373(std::optional<T1> &a0) {
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
T1 const &f374(const std::optional<T1> &a0) {
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
T1 &f375(std::optional<T1> &a0) {
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
T1 &&f376(std::optional<T1> &&a0) {
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
T1 const &&f377(const std::optional<T1> &&a0) {
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
T1 const &f378(const std::optional<T1> &a0) {
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
T1 &f379(std::optional<T1> &a0) {
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
T1 &&f380(std::optional<T1> &&a0) {
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
T1 const &&f381(const std::optional<T1> &&a0) {
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
void f382(std::optional<T1> &a0) {
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
bool f383(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f384(const std::optional<T1> &a0) {
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
T1 &f385(const std::optional<T1> &a0) {
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
T1 &f386(const std::optional<T1> &a0) {
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
bool f387(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1 const> f388(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f389(std::optional<T1> &a0) {
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
T1 const &f390(const std::optional<T1> &a0) {
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
T1 &f391(std::optional<T1> &a0) {
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
T1 &&f392(std::optional<T1> &&a0) {
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
T1 const &&f393(const std::optional<T1> &&a0) {
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
T1 const &f394(const std::optional<T1> &a0) {
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
T1 &f395(std::optional<T1> &a0) {
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
T1 &&f396(std::optional<T1> &&a0) {
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
T1 const &&f397(const std::optional<T1> &&a0) {
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
void f398(std::optional<T1> &a0) {
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
bool f399(const std::optional<T1> &a0) {
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
std::add_pointer_t<T1> f400(const std::optional<T1> &a0) {
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
T1 &f401(const std::optional<T1> &a0) {
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
T1 &f402(const std::optional<T1> &a0) {
  return a0.value();
}
