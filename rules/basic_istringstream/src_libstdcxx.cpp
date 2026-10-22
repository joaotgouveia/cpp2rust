#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f45(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> *
f46(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f47(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f48(std::basic_istringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f49(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_istringstream<T1, T2, T3> &(*a1)(
        std::basic_istringstream<T1, T2, T3> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f50(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f51(std::basic_istringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f52(std::basic_istringstream<T1, T2, T3> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f53(std::basic_istringstream<T1, T2, T3> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f54(std::basic_istringstream<T1, T2, T3> &a0, unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f55(std::basic_istringstream<T1, T2, T3> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f56(std::basic_istringstream<T1, T2, T3> &a0, unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f57(std::basic_istringstream<T1, T2, T3> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f58(std::basic_istringstream<T1, T2, T3> &a0, unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f59(std::basic_istringstream<T1, T2, T3> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f60(std::basic_istringstream<T1, T2, T3> &a0, unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f61(std::basic_istringstream<T1, T2, T3> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f62(std::basic_istringstream<T1, T2, T3> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f63(std::basic_istringstream<T1, T2, T3> &a0, long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f64(std::basic_istringstream<T1, T2, T3> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f65(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f66(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f67(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2,
    typename std::basic_istringstream<T1, T2, T3>::char_type a3) {
  return a0.get(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f68(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f69(std::basic_istringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> &a1,
    typename std::basic_istringstream<T1, T2, T3>::char_type a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f70(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f71(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2,
    typename std::basic_istringstream<T1, T2, T3>::char_type a3) {
  return a0.getline(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f72(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f73(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1,
    typename std::basic_istringstream<T1, T2, T3>::int_type a2) {
  return a0.ignore(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f74(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f75(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f76(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f77(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type a1) {
  return a0.putback(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f78(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.unget();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f79(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekg(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f80(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::off_type a1,
    typename std::basic_istringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekg(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *
f81(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f82(std::basic_istringstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}
