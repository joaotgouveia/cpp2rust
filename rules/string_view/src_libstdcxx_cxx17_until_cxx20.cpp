#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f2(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!= <T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f3(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator!=(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f4(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!=(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f5(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator< <T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f6(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator<(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f7(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f8(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<= <T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f9(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator<=(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f10(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<=(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f11(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator== <T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f12(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator==(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f13(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator==(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f14(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator><T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f15(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f16(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f17(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>= <T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f18(std::basic_string_view<T1, T2> a0, const T3 &a1) {
  return std::operator>=(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              (std::is_convertible<const T3 &,
                                   std::basic_string_view<T1, T2>>::value &&
               !(std::is_same<T3, std::basic_string_view<T1, T2>>::value)),
              int>::type T4 = 0>
bool f19(const T3 &a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>=(a0, std::move(a1));
}
