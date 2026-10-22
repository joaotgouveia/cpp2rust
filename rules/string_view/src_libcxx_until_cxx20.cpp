#include "../common/init.h"
#include "../common/traits.h"
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f19(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2, int T3 = 1>
bool f20(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2, int T3 = 2>
bool f21(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f22(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2, int T3 = 1>
bool f23(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2, int T3 = 2>
bool f24(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f25(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, int T3 = 1>
bool f26(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, int T3 = 2>
bool f27(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f28(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2, int T3 = 1>
bool f29(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2, int T3 = 2>
bool f30(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f31(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, int T3 = 1>
bool f32(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, int T3 = 2>
bool f33(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return a0 >= a1;
}
