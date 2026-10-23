#include <iosfwd>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f2(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f3(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f4(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f5(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f6(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
