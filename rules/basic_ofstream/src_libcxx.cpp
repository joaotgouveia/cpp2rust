#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> f73(const char *a0, unsigned int a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> f74(const std::string &a0, unsigned int a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f75(std::basic_ofstream<T1, T2> &a0, const char *a1, unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
void f76(std::basic_ofstream<T1, T2> &a0, const std::string &a1,
         unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
unsigned int f77(const std::basic_ofstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f78(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f79(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f80(const std::basic_ofstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f81(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f82(const std::basic_ofstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f83(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f84(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f85(std::basic_ofstream<T1, T2> &a0, unsigned int a1,
                 unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f86(std::basic_ofstream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
