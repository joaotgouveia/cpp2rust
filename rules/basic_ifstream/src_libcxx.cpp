#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> f86(const char *a0, unsigned int a1) {
  return std::basic_ifstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> f87(const std::string &a0, unsigned int a1) {
  return std::basic_ifstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f88(std::basic_ifstream<T1, T2> &a0, const char *a1, unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
void f89(std::basic_ifstream<T1, T2> &a0, const std::string &a1,
         unsigned int a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
unsigned int f90(const std::basic_ifstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f91(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f92(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
unsigned int f93(const std::basic_ifstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f94(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
unsigned int f95(const std::basic_ifstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
unsigned int f96(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
unsigned int f97(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
unsigned int f98(std::basic_ifstream<T1, T2> &a0, unsigned int a1,
                 unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f99(std::basic_ifstream<T1, T2> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
