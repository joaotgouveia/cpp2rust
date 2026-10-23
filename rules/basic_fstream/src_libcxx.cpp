#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
std::basic_fstream<T1, T2> f110(const char *a0, unsigned int a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> f111(const std::string &a0, unsigned int a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f112(std::basic_fstream<T1, T2> &a0, const char *a1, unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
void f113(std::basic_fstream<T1, T2> &a0, const std::string &a1,
          unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
unsigned int f114(const std::basic_fstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f115(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f116(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f117(const std::basic_fstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f118(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f119(const std::basic_fstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f120(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f121(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f122(std::basic_fstream<T1, T2> &a0, unsigned int a1,
                  unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f123(std::basic_fstream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
