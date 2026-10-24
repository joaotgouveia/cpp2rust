#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f41(typename std::basic_istringstream<T1, T2, T3>::openmode a0) {
  return std::basic_istringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f42(const std::basic_string<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f43(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_istringstream<T1, T2, T3> &(*a1)(
        std::basic_istringstream<T1, T2, T3> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f44(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f45(std::basic_istringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f46(std::basic_istringstream<T1, T2, T3> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f47(std::basic_istringstream<T1, T2, T3> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f48(std::basic_istringstream<T1, T2, T3> &a0, unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f49(std::basic_istringstream<T1, T2, T3> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f50(std::basic_istringstream<T1, T2, T3> &a0, unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f51(std::basic_istringstream<T1, T2, T3> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f52(std::basic_istringstream<T1, T2, T3> &a0, unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f53(std::basic_istringstream<T1, T2, T3> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f54(std::basic_istringstream<T1, T2, T3> &a0, unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f55(std::basic_istringstream<T1, T2, T3> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f56(std::basic_istringstream<T1, T2, T3> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f57(std::basic_istringstream<T1, T2, T3> &a0, long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f58(std::basic_istringstream<T1, T2, T3> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f59(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f60(std::basic_istringstream<T1, T2, T3> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f61(std::basic_istringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2,
    T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f62(std::basic_istringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f63(std::basic_istringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> &a1,
    T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f64(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f65(std::basic_istringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2,
    T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f66(std::basic_istringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f67(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1,
    typename std::basic_istringstream<T1, T2, T3>::int_type a2) {
  return a0.ignore(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f68(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f69(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f70(std::basic_istringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f71(std::basic_istringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f72(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.unget();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f73(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekg(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f74(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::off_type a1,
    typename std::basic_istringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekg(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f75(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f76(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f77(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f78(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f79(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags f80(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f81(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f82(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f83(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f84(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(std::move(a1));
}
