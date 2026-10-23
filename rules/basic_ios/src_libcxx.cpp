#include <ios>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
unsigned int f38(const std::basic_ios<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f39(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f40(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f41(const std::basic_ios<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f42(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
std::basic_ios<T1, T2> f43(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_ios<T1, T2>(a0);
}

template <typename T1, typename T2>
unsigned int f44(const std::basic_ios<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f45(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f46(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f47(std::basic_ios<T1, T2> &a0, unsigned int a1, unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f48(std::basic_ios<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
