#include "../common/init.h"
#include "../common/traits.h"
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2> bool f45(std::basic_ofstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f46(std::basic_ofstream<T1, T2> &a0,
    std::basic_ofstream<T1, T2> &(*a1)(std::basic_ofstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f47(std::basic_ofstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f48(std::basic_ofstream<T1, T2> &a0,
                                 std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f49(std::basic_ofstream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f50(std::basic_ofstream<T1, T2> &a0,
                                 unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f51(std::basic_ofstream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f52(std::basic_ofstream<T1, T2> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f53(std::basic_ofstream<T1, T2> &a0,
                                 unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f54(std::basic_ofstream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f55(std::basic_ofstream<T1, T2> &a0,
                                 unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f56(std::basic_ofstream<T1, T2> &a0,
                                 long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f57(std::basic_ofstream<T1, T2> &a0,
                                 unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f58(std::basic_ofstream<T1, T2> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f59(std::basic_ofstream<T1, T2> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f60(std::basic_ofstream<T1, T2> &a0,
                                 long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f61(std::basic_ofstream<T1, T2> &a0,
                                 const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f62(std::basic_ofstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f63(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::char_type a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f64(std::basic_ofstream<T1, T2> &a0,
    const typename std::basic_ofstream<T1, T2>::char_type *a1,
    std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f65(std::basic_ofstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f66(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f67(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::off_type a1,
    typename std::basic_ofstream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f68(const std::basic_ofstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f69(std::basic_ofstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}
