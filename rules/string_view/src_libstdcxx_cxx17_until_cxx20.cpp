#include <iosfwd>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f2(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f3(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f4(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f5(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f6(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f7(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f8(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f9(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f10(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f11(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f12(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f13(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f14(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f15(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f16(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}
