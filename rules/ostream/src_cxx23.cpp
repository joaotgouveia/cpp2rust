#include <barrier>
#include <ostream>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f1(std::basic_ostream<T1, T2> &a0) {
  return std::emit_on_flush(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f2(std::basic_ostream<T1, T2> &a0) {
  return std::endl(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f3(std::basic_ostream<T1, T2> &a0) {
  return std::ends(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f4(std::basic_ostream<T1, T2> &a0) {
  return std::flush(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f5(std::basic_ostream<T1, T2> &a0) {
  return std::flush_emit(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f6(std::basic_ostream<T1, T2> &a0) {
  return std::noemit_on_flush(a0);
}

template <typename T1>
std::basic_ostream<char, T1> &f7(std::basic_ostream<char, T1> &a0,
                                 const char *a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f8(std::basic_ostream<char, T1> &a0,
                                 const unsigned char *a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f9(std::basic_ostream<char, T1> &a0,
                                 const signed char *a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f10(std::basic_ostream<char, T1> &a0, char a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f11(std::basic_ostream<char, T1> &a0,
                                  unsigned char a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f12(std::basic_ostream<char, T1> &a0,
                                  signed char a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2> T1 &&f13(T1 &&a0, const T2 &a1) {
  return std::operator<<(std::forward<T1>(a0), a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f14(std::basic_ostream<T1, T2> &a0,
                                const char *a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f15(std::basic_ostream<T1, T2> &a0, const T1 *a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0, char a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f17(std::basic_ostream<T1, T2> &a0, T1 a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename... T1>
void f18(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::print(a0, std::forward<std::format_string<T1...>>(a1),
                    std::forward<T1>(a2)...);
}

void f19(std::ostream &a0) { return std::println(a0); }

template <typename... T1>
void f20(std::ostream &a0, std::format_string<T1...> a1, T1 &&...a2) {
  return std::println(a0, std::forward<std::format_string<T1...>>(a1),
                      std::forward<T1>(a2)...);
}

void f21(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_nonunicode(a0, std::move(a1), std::move(a2));
}

void f22(std::ostream &a0, std::string_view a1, std::format_args a2) {
  return std::vprint_unicode(a0, std::move(a1), std::move(a2));
}
