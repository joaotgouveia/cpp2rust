#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f9(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 1>
bool f10(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f11(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f12(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f13(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f14(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 1>
bool f15(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f16(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f17(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 1>
bool f18(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f19(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f20(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 1>
bool f21(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f22(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f23(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 1>
bool f24(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, int T3 = 2>
bool f25(std::basic_string_view<T1, T2> a0, std::basic_string_view<T1, T2> a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
