#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_istringstream<T1, T2, T3>::string_type;

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> f91(unsigned int a0) {
  return std::basic_istringstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f92(const std::basic_string<T1, T2, T3> &a0, unsigned int a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f93(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f94(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f95(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f96(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f97(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f98(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
unsigned int f99(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f100(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f101(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1,
                  unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f102(std::basic_istringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
