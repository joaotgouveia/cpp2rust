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
f302(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
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
std::basic_string<T1, T2, T3> f303(const T4 &a0, const T3 &a1) {
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
std::basic_string<T1, T2, T3> &f304(std::basic_string<T1, T2, T3> &a0,
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
std::basic_string<T1, T2, T3> &f305(std::basic_string<T1, T2, T3> &a0,
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
std::basic_string<T1, T2, T3> &f306(std::basic_string<T1, T2, T3> &a0,
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
f307(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
std::basic_string<T1, T2, T3> &f308(std::basic_string<T1, T2, T3> &a0,
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
f309(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f310(std::basic_string<T1, T2, T3> &a0,
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
f311(std::basic_string<T1, T2, T3> &a0,
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
f312(std::basic_string<T1, T2, T3> &a0,
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
f313(std::basic_string<T1, T2, T3> &a0,
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
f314(std::basic_string<T1, T2, T3> &a0,
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
f315(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f316(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f317(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f318(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f319(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
f320(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
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
int f321(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
int f322(const std::basic_string<T1, T2, T3> &a0,
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
int f323(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(std::move(a1), std::move(a2), a3,
                                 std::move(a4), std::move(a5));
}
