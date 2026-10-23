#include <bitset>
#include <ios>
#include <memory>
#include <ostream>
#include <streambuf>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
unsigned int f62(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f63(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f64(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f65(const std::basic_ostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f66(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f67(const std::basic_ostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f68(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f69(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f70(std::basic_ostream<T1, T2> &a0, unsigned int a1,
                 unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f71(std::basic_ostream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f72(std::basic_ostream<T1, T2> &a0) {
  return std::endl<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f73(std::basic_ostream<T1, T2> &a0) {
  return std::ends<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f74(std::basic_ostream<T1, T2> &a0) {
  return std::flush<T1, T2>(a0);
}

template <typename T1>
std::basic_ostream<char, T1> &f75(std::basic_ostream<char, T1> &a0,
                                  const char *a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f76(std::basic_ostream<char, T1> &a0,
                                  const unsigned char *a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f77(std::basic_ostream<char, T1> &a0,
                                  const signed char *a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f78(std::basic_ostream<char, T1> &a0, char a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f79(std::basic_ostream<char, T1> &a0,
                                  unsigned char a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1>
std::basic_ostream<char, T1> &f80(std::basic_ostream<char, T1> &a0,
                                  signed char a1) {
  return std::operator<< <T1>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f81(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f82(std::basic_ostream<T1, T2> &a0,
                                const char *a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f83(std::basic_ostream<T1, T2> &a0, const T1 *a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f84(std::basic_ostream<T1, T2> &a0,
                                std::basic_string_view<T1, T2> a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f85(std::basic_ostream<T1, T2> &a0, char a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f86(std::basic_ostream<T1, T2> &a0, T1 a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3>
std::basic_ostream<T1, T2> &f87(std::basic_ostream<T1, T2> &a0,
                                const std::bitset<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f88(std::basic_ostream<T1, T2> &a0,
                                const std::shared_ptr<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f89(std::basic_ostream<T1, T2> &a0,
                                const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<true, int>::type T5 = 0>
std::basic_ostream<T1, T2> &f90(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}
