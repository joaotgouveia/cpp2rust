#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f23(const T3 &a0) {
  return std::basic_stringbuf<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f24(std::ios_base::openmode a0, const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f25(std::basic_string<T1, T2, T3> &&a0,
                                     std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> f26(std::basic_stringbuf<T1, T2, T3> &&a0,
                                     const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
T3 f27(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f28(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f29(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f30(std::basic_stringbuf<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f31(const std::basic_string<T1, T2, T4> &a0,
                                     const T3 &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f32(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1, const T3 &a2) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1, a2);
}
