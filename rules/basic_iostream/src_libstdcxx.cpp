#include <istream>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f33(std::basic_iostream<T1, T2> &a0,
    std::basic_iostream<T1, T2> &(*a1)(std::basic_iostream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f34(std::basic_iostream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f35(std::basic_iostream<T1, T2> &a0,
                                 std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f36(std::basic_iostream<T1, T2> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f37(std::basic_iostream<T1, T2> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f38(std::basic_iostream<T1, T2> &a0,
                                 unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f39(std::basic_iostream<T1, T2> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f40(std::basic_iostream<T1, T2> &a0,
                                 unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f41(std::basic_iostream<T1, T2> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f42(std::basic_iostream<T1, T2> &a0,
                                 unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f43(std::basic_iostream<T1, T2> &a0,
                                 long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f44(std::basic_iostream<T1, T2> &a0,
                                 unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f45(std::basic_iostream<T1, T2> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f46(std::basic_iostream<T1, T2> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f47(std::basic_iostream<T1, T2> &a0,
                                 long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f48(std::basic_iostream<T1, T2> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f49(std::basic_iostream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f50(std::basic_iostream<T1, T2> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f51(std::basic_iostream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f52(std::basic_iostream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f53(std::basic_iostream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f54(std::basic_iostream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f55(std::basic_iostream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f56(std::basic_iostream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f57(std::basic_iostream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_iostream<T1, T2>::int_type a2) {
  return a0.ignore(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f58(std::basic_iostream<T1, T2> &a0,
                                 std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f59(std::basic_iostream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f60(std::basic_iostream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f61(std::basic_iostream<T1, T2> &a0, T1 a1) {
  return a0.putback(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f62(std::basic_iostream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f63(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::pos_type a1) {
  return a0.seekg(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f64(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::off_type a1,
    typename std::basic_iostream<T1, T2>::seekdir a2) {
  return a0.seekg(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::iostate
f65(const std::basic_iostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f66(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f67(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::iostate
f68(const std::basic_iostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f69(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f70(const std::basic_iostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f71(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f72(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f73(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1,
    typename std::basic_iostream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f74(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f75(std::basic_iostream<T1, T2> &a0,
    std::basic_iostream<T1, T2> &(*a1)(std::basic_iostream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f76(std::basic_iostream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f77(std::basic_iostream<T1, T2> &a0,
                                 std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f78(std::basic_iostream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f79(std::basic_iostream<T1, T2> &a0,
                                 unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f80(std::basic_iostream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f81(std::basic_iostream<T1, T2> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f82(std::basic_iostream<T1, T2> &a0,
                                 unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f83(std::basic_iostream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f84(std::basic_iostream<T1, T2> &a0,
                                 unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f85(std::basic_iostream<T1, T2> &a0,
                                 long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f86(std::basic_iostream<T1, T2> &a0,
                                 unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f87(std::basic_iostream<T1, T2> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f88(std::basic_iostream<T1, T2> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f89(std::basic_iostream<T1, T2> &a0,
                                 long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f90(std::basic_iostream<T1, T2> &a0,
                                 const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f91(std::basic_iostream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f92(std::basic_iostream<T1, T2> &a0, T1 a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f93(std::basic_iostream<T1, T2> &a0, T1 const *a1,
                                 std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f94(std::basic_iostream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f95(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &
f96(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::off_type a1,
    typename std::basic_iostream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
}
