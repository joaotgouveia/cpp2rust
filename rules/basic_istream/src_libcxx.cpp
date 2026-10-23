#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
unsigned int f75(const std::basic_istream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f76(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f77(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f78(const std::basic_istream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f79(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f80(const std::basic_istream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f81(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f82(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f83(std::basic_istream<T1, T2> &a0, unsigned int a1,
                 unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f84(std::basic_istream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
