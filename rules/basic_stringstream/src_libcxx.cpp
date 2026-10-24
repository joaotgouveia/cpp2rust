#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_stringstream<T1, T2, T3>::string_type;

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> f116(unsigned int a0) {
  return std::basic_stringstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f117(const std::basic_string<T1, T2, T3> &a0, unsigned int a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4 = T1,
          std::__enable_if_t<std::is_same<T1, char>::value, int> T5 = 0>
std::basic_stringstream<T1, T2, T3> &
f118(std::basic_stringstream<T1, T2, T3> &a0, std::streamsize a1, T1 a2) {
  return a0.ignore(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
unsigned int f119(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f120(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f121(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f122(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f123(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f124(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
unsigned int f125(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f126(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
unsigned int f127(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1,
                  unsigned int a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f128(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.unsetf(a1);
}
