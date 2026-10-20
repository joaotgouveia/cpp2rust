#include "../common/init.h"
#include "../common/traits.h"
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ofstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ofstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ofstream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ofstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ofstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ofstream<T1, T2>::off_type;

template <typename T1, typename T2> std::basic_ofstream<T1, T2> f1() {
  return std::basic_ofstream<T1, T2>();
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2>
f2(const char *a0, typename std::basic_ofstream<T1, T2>::openmode a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2>
f3(const std::string &a0, typename std::basic_ofstream<T1, T2>::openmode a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> f4(std::basic_ofstream<T1, T2> &&a0) {
  return std::basic_ofstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f5(std::basic_ofstream<T1, T2> &a0,
                                std::basic_ofstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f6(std::basic_ofstream<T1, T2> &a0, std::basic_ofstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_filebuf<typename std::basic_ofstream<T1, T2>::char_type,
                   typename std::basic_ofstream<T1, T2>::traits_type> *
f7(const std::basic_ofstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2> bool f8(std::basic_ofstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
bool f9(const std::basic_ofstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
void f10(std::basic_ofstream<T1, T2> &a0, const char *a1,
         typename std::basic_ofstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
void f11(std::basic_ofstream<T1, T2> &a0, const std::string &a1,
         typename std::basic_ofstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2> void f12(std::basic_ofstream<T1, T2> &a0) {
  return a0.close();
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f13(std::basic_ofstream<T1, T2> &a0,
    std::basic_ofstream<T1, T2> &(*a1)(std::basic_ofstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f14(std::basic_ofstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f15(std::basic_ofstream<T1, T2> &a0,
                                 std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f16(std::basic_ofstream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f17(std::basic_ofstream<T1, T2> &a0,
                                 unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f18(std::basic_ofstream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f19(std::basic_ofstream<T1, T2> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f20(std::basic_ofstream<T1, T2> &a0,
                                 unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f21(std::basic_ofstream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f22(std::basic_ofstream<T1, T2> &a0,
                                 unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f23(std::basic_ofstream<T1, T2> &a0,
                                 long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f24(std::basic_ofstream<T1, T2> &a0,
                                 unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f25(std::basic_ofstream<T1, T2> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f26(std::basic_ofstream<T1, T2> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f27(std::basic_ofstream<T1, T2> &a0,
                                 long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f28(std::basic_ofstream<T1, T2> &a0,
                                 const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f29(std::basic_ofstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f30(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::char_type a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f31(std::basic_ofstream<T1, T2> &a0,
    const typename std::basic_ofstream<T1, T2>::char_type *a1,
    std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f32(std::basic_ofstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::pos_type
f33(std::basic_ofstream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f34(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &
f35(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::off_type a1,
    typename std::basic_ofstream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
}

template <typename T1, typename T2>
bool f36(const std::basic_ofstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f37(const std::basic_ofstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::iostate
f38(const std::basic_ofstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f39(std::basic_ofstream<T1, T2> &a0,
         typename std::basic_ofstream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f40(std::basic_ofstream<T1, T2> &a0,
         typename std::basic_ofstream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
bool f41(const std::basic_ofstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f42(const std::basic_ofstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f43(const std::basic_ofstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f44(const std::basic_ofstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::iostate
f45(const std::basic_ofstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f46(std::basic_ofstream<T1, T2> &a0,
         typename std::basic_ofstream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f47(const std::basic_ofstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f48(std::basic_ofstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::char_type
f49(const std::basic_ofstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::char_type
f50(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f51(std::basic_ofstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f52(const std::basic_ofstream<T1, T2> &a0,
         typename std::basic_ofstream<T1, T2>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::char_type
f53(const std::basic_ofstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f54(std::basic_ofstream<T1, T2> &a0,
         void (*a1)(typename std::basic_ofstream<T1, T2>::event,
                    std::basic_ofstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::fmtflags
f55(const std::basic_ofstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::fmtflags
f56(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::fmtflags
f57(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_ofstream<T1, T2>::fmtflags
f58(std::basic_ofstream<T1, T2> &a0,
    typename std::basic_ofstream<T1, T2>::fmtflags a1,
    typename std::basic_ofstream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f59(std::basic_ofstream<T1, T2> &a0,
         typename std::basic_ofstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::streamsize f60(const std::basic_ofstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f61(std::basic_ofstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f62(const std::basic_ofstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f63(std::basic_ofstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f64(bool a0) {
  return std::basic_ofstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f65(const std::basic_ofstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f66() {
  return std::basic_ofstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f67(std::basic_ofstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f68(std::basic_ofstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}
