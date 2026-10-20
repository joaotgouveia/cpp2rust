#include "../common/init.h"
#include "../common/traits.h"
#include <ostream>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f1(std::basic_ostream<T1, T2> &a0) {
  return std::endl<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f2(std::basic_ostream<T1, T2> &a0) {
  return std::ends<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f3(std::basic_ostream<T1, T2> &a0) {
  return std::flush<T1, T2>(a0);
}

template <typename T1>
std::basic_ostream<char, T1> &f4(std::basic_ostream<char, T1> &a0,
                                 const char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f5(std::basic_ostream<char, T1> &a0,
                                 const unsigned char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f6(std::basic_ostream<char, T1> &a0,
                                 const signed char *a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f7(std::basic_ostream<char, T1> &a0, char a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f8(std::basic_ostream<char, T1> &a0,
                                 unsigned char a1) {
  return a0 << a1;
}

template <typename T1>
std::basic_ostream<char, T1> &f9(std::basic_ostream<char, T1> &a0,
                                 signed char a1) {
  return a0 << a1;
}

template <typename T1, typename T2> T1 &&f10(T1 &&a0, const T2 &a1) {
  return std::forward<T1>(a0) << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f11(std::basic_ostream<T1, T2> &a0,
                                const char *a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f12(std::basic_ostream<T1, T2> &a0, const T1 *a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f13(std::basic_ostream<T1, T2> &a0, char a1) {
  return a0 << a1;
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f14(std::basic_ostream<T1, T2> &a0, T1 a1) {
  return a0 << a1;
}
