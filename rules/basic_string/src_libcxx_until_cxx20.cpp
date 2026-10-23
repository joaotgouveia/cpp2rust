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
f256(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
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
std::basic_string<T1, T2, T3> f257(const T4 &a0, const T3 &a1) {
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
std::basic_string<T1, T2, T3> &f258(std::basic_string<T1, T2, T3> &a0,
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
std::basic_string<T1, T2, T3> &f259(std::basic_string<T1, T2, T3> &a0,
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
std::basic_string<T1, T2, T3> &f260(std::basic_string<T1, T2, T3> &a0,
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
f261(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &f262(std::basic_string<T1, T2, T3> &a0,
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
f263(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f264(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(a1, a2);
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
f265(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f266(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
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
f267(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f268(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f269(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f270(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template rfind<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f271(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f272(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f273(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
typename std::basic_string<T1, T2, T3>::size_type
f274(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
int f275(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__can_be_converted_to_string_view_v<T1, T2, T4>, int>
              T5 = 0,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
int f276(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(a1, a2, a3);
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
int f277(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}
