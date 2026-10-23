#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f38(typename std::basic_ostringstream<T1, T2, T3>::openmode a0) {
  return std::basic_ostringstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f39(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::openmode a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f40(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_ostringstream<T1, T2, T3> &(*a1)(
        std::basic_ostringstream<T1, T2, T3> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f41(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f42(std::basic_ostringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f43(std::basic_ostringstream<T1, T2, T3> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f44(std::basic_ostringstream<T1, T2, T3> &a0, unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f45(std::basic_ostringstream<T1, T2, T3> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f46(std::basic_ostringstream<T1, T2, T3> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f47(std::basic_ostringstream<T1, T2, T3> &a0, unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f48(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f49(std::basic_ostringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f50(std::basic_ostringstream<T1, T2, T3> &a0, long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f51(std::basic_ostringstream<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f52(std::basic_ostringstream<T1, T2, T3> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f53(std::basic_ostringstream<T1, T2, T3> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f54(std::basic_ostringstream<T1, T2, T3> &a0, long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f55(std::basic_ostringstream<T1, T2, T3> &a0, const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f56(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f57(std::basic_ostringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.put(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f58(std::basic_ostringstream<T1, T2, T3> &a0, T1 const *a1,
    std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f59(std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.flush();
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f60(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f61(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::off_type a1,
    typename std::basic_ostringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekp(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::iostate
f62(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f63(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f64(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::iostate
f65(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f66(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f67(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f68(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f69(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f70(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f71(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(a1);
}
