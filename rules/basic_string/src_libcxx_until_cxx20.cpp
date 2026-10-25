#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3>
f298(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> f299(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &f300(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator= <T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &f301(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator+= <T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &f302(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template append<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &
f303(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &f304(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template assign<T4>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &
f305(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f306(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(std::move(a1), a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &
f307(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(std::move(a1), a2, std::move(a3),
                                std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f308(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> &
f309(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3,
                                 std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f310(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f311(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f312(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template rfind<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f313(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f314(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f315(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_not_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f316(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_not_of<T4>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
int f317(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
int f318(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(std::move(a1), std::move(a2), a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__can_be_converted_to_string_view_v<T1, T2, T4> &&
                           !std::is_same<std::__remove_cvref_t<T4>,
                                         std::basic_string<T1, T2, T3>>::value,
                       int>
        T5 = 0,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
int f319(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(std::move(a1), std::move(a2), a3,
                                 std::move(a4), std::move(a5));
}
