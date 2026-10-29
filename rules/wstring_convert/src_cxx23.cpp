#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t1 = std::wstring_convert<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t2 = typename std::wstring_convert<T1, T2, T3, T4>::byte_string;

template <typename T1, typename T2, typename T3, typename T4>
using t3 = typename std::wstring_convert<T1, T2, T3, T4>::wide_string;

template <typename T1, typename T2, typename T3, typename T4>
using t4 = typename std::wstring_convert<T1, T2, T3, T4>::state_type;

template <typename T1, typename T2, typename T3, typename T4>
using t5 = typename std::wstring_convert<T1, T2, T3, T4>::int_type;

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4> f1() {
  return {};
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4> f2(T1 *a0) {
  return std::wstring_convert<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f3(T1 *a0, typename std::wstring_convert<T1, T2, T3, T4>::state_type a1) {
  return {
      a0,
      std::forward<typename std::wstring_convert<T1, T2, T3, T4>::state_type>(
          a1)};
}

template <typename T1, typename T2, typename T3, typename T4>
std::wstring_convert<T1, T2, T3, T4>
f4(const std::basic_string<char, std::char_traits<char>, T4> &a0,
   const std::basic_string<T2, std::char_traits<T2>, T3> &a1) {
  return std::wstring_convert<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, std::char_traits<T2>, T3>
f5(std::wstring_convert<T1, T2, T3, T4> &a0, char a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, std::char_traits<T2>, T3>
f6(std::wstring_convert<T1, T2, T3, T4> &a0, const char *a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, std::char_traits<T2>, T3>
f7(std::wstring_convert<T1, T2, T3, T4> &a0,
   const std::basic_string<char, std::char_traits<char>, T4> &a1) {
  return a0.from_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, std::char_traits<T2>, T3>
f8(std::wstring_convert<T1, T2, T3, T4> &a0, const char *a1, const char *a2) {
  return a0.from_bytes(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<char, std::char_traits<char>, T4>
f9(std::wstring_convert<T1, T2, T3, T4> &a0, T2 a1) {
  return a0.to_bytes(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<char, std::char_traits<char>, T4>
f10(std::wstring_convert<T1, T2, T3, T4> &a0, const T2 *a1) {
  return a0.to_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<char, std::char_traits<char>, T4>
f11(std::wstring_convert<T1, T2, T3, T4> &a0,
    const std::basic_string<T2, std::char_traits<T2>, T3> &a1) {
  return a0.to_bytes(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<char, std::char_traits<char>, T4>
f12(std::wstring_convert<T1, T2, T3, T4> &a0, const T2 *a1, const T2 *a2) {
  return a0.to_bytes(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::size_t f13(const std::wstring_convert<T1, T2, T3, T4> &a0) {
  return a0.converted();
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::wstring_convert<T1, T2, T3, T4>::state_type
f14(const std::wstring_convert<T1, T2, T3, T4> &a0) {
  return a0.state();
}
