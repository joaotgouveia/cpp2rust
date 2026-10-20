#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <ostream>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ostream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ostream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ostream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ostream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ostream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ostream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t7 = typename std::basic_ostream<T1, T2>::sentry;

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f1(std::basic_ostream<T1, T2> &a0,
   std::basic_ostream<T1, T2> &(*a1)(std::basic_ostream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f2(std::basic_ostream<T1, T2> &a0,
   std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f3(std::basic_ostream<T1, T2> &a0,
                               std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f4(std::basic_ostream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f5(std::basic_ostream<T1, T2> &a0,
                               unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f6(std::basic_ostream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f7(std::basic_ostream<T1, T2> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f8(std::basic_ostream<T1, T2> &a0,
                               unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f9(std::basic_ostream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f10(std::basic_ostream<T1, T2> &a0,
                                unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f11(std::basic_ostream<T1, T2> &a0, long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f12(std::basic_ostream<T1, T2> &a0,
                                unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f13(std::basic_ostream<T1, T2> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f14(std::basic_ostream<T1, T2> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f15(std::basic_ostream<T1, T2> &a0,
                                long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0,
                                const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f17(std::basic_ostream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f18(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::char_type a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f19(std::basic_ostream<T1, T2> &a0,
    const typename std::basic_ostream<T1, T2>::char_type *a1,
    std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f20(std::basic_ostream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::pos_type
f21(std::basic_ostream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f22(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f23(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::off_type a1,
    typename std::basic_ostream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
}

template <typename T1, typename T2>
bool f24(const std::basic_ostream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f25(const std::basic_ostream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f26(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f27(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f28(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
bool f29(const std::basic_ostream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f30(const std::basic_ostream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f31(const std::basic_ostream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f32(const std::basic_ostream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::iostate
f33(const std::basic_ostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f34(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f35(const std::basic_ostream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f36(std::basic_ostream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f37(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f38(std::basic_ostream<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::char_type
f39(const std::basic_ostream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::char_type
f40(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f41(std::basic_ostream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f42(const std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::char_type
f43(const std::basic_ostream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f44(std::basic_ostream<T1, T2> &a0,
         void (*a1)(typename std::basic_ostream<T1, T2>::event,
                    std::basic_ostream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f45(const std::basic_ostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f46(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f47(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_ostream<T1, T2>::fmtflags
f48(std::basic_ostream<T1, T2> &a0,
    typename std::basic_ostream<T1, T2>::fmtflags a1,
    typename std::basic_ostream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f49(std::basic_ostream<T1, T2> &a0,
         typename std::basic_ostream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::streamsize f50(const std::basic_ostream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f51(std::basic_ostream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f52(const std::basic_ostream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f53(std::basic_ostream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f54(bool a0) {
  return std::basic_ostream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f55(const std::basic_ostream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f56() {
  return std::basic_ostream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f57(std::basic_ostream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f58(std::basic_ostream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}
