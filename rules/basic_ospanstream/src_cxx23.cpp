#include <algorithm>
#include <ios>
#include <iosfwd>
#include <span>
#include <spanstream>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ospanstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ospanstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ospanstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ospanstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ospanstream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ospanstream<T1, T2>::traits_type;

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2>
f1(std::span<T1> a0, typename std::basic_ospanstream<T1, T2>::openmode a1) {
  return std::basic_ospanstream<T1, T2>(
      std::forward<std::span<T1>>(a0),
      std::forward<typename std::basic_ospanstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> f2(std::basic_ospanstream<T1, T2> &&a0) {
  return std::basic_ospanstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f3(std::basic_ospanstream<T1, T2> &a0,
                                   std::basic_ospanstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f4(std::basic_ospanstream<T1, T2> &a0,
        std::basic_ospanstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> *f5(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::span<T1> f6(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.span();
}

template <typename T1, typename T2>
void f7(std::basic_ospanstream<T1, T2> &a0, std::span<T1> a1) {
  return a0.span(std::forward<std::span<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f8(std::basic_ospanstream<T1, T2> &a0,
   std::basic_ospanstream<T1, T2> &(*a1)(std::basic_ospanstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f9(std::basic_ospanstream<T1, T2> &a0,
   std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f10(std::basic_ospanstream<T1, T2> &a0,
                                    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f11(std::basic_ospanstream<T1, T2> &a0,
                                    long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f12(std::basic_ospanstream<T1, T2> &a0,
                                    unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f13(std::basic_ospanstream<T1, T2> &a0,
                                    bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f14(std::basic_ospanstream<T1, T2> &a0,
                                    short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f15(std::basic_ospanstream<T1, T2> &a0,
                                    unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f16(std::basic_ospanstream<T1, T2> &a0,
                                    int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f17(std::basic_ospanstream<T1, T2> &a0,
                                    unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f18(std::basic_ospanstream<T1, T2> &a0,
                                    long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f19(std::basic_ospanstream<T1, T2> &a0,
                                    unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f20(std::basic_ospanstream<T1, T2> &a0,
                                    double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f21(std::basic_ospanstream<T1, T2> &a0,
                                    float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f22(std::basic_ospanstream<T1, T2> &a0,
                                    long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f23(std::basic_ospanstream<T1, T2> &a0,
                                    const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f24(std::basic_ospanstream<T1, T2> &a0,
                                    std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f25(std::basic_ospanstream<T1, T2> &a0,
                                    const volatile void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f26(std::basic_ospanstream<T1, T2> &a0,
                                    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f27(std::basic_ospanstream<T1, T2> &a0, T1 a1) {
  return a0.put(std::move(a1));
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f28(std::basic_ospanstream<T1, T2> &a0,
                                    T1 const *a1, std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &f29(std::basic_ospanstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::pos_type
f30(std::basic_ospanstream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f31(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::pos_type a1) {
  return a0.seekp(
      std::forward<typename std::basic_ospanstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f32(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::off_type a1,
    typename std::basic_ospanstream<T1, T2>::seekdir a2) {
  return a0.seekp(
      std::forward<typename std::basic_ospanstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_ospanstream<T1, T2>::seekdir>(a2));
}

template <typename T1, typename T2>
bool f33(const std::basic_ospanstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f34(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate
f35(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f36(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_ospanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f37(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_ospanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
bool f38(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f39(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f40(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f41(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate
f42(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f43(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_ospanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f44(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f45(std::basic_ospanstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f46(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f47(std::basic_ospanstream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f48(std::basic_ospanstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f49(const std::basic_ospanstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f50(const std::basic_ospanstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f51(std::basic_ospanstream<T1, T2> &a0,
         void (*a1)(typename std::basic_ospanstream<T1, T2>::event,
                    std::basic_ospanstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f52(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f53(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_ospanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f54(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_ospanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f55(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1,
    typename std::basic_ospanstream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_ospanstream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_ospanstream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f56(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_ospanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f57(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f58(std::basic_ospanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f59(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f60(std::basic_ospanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f61(bool a0) {
  return std::basic_ospanstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f62(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f63() {
  return std::basic_ospanstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f64(std::basic_ospanstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f65(std::basic_ospanstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f66() {
  return std::basic_ospanstream<T1, T2>::boolalpha;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f67() {
  return std::basic_ospanstream<T1, T2>::dec;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f68() {
  return std::basic_ospanstream<T1, T2>::fixed;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f69() {
  return std::basic_ospanstream<T1, T2>::hex;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f70() {
  return std::basic_ospanstream<T1, T2>::internal;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f71() {
  return std::basic_ospanstream<T1, T2>::left;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f72() {
  return std::basic_ospanstream<T1, T2>::oct;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f73() {
  return std::basic_ospanstream<T1, T2>::right;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f74() {
  return std::basic_ospanstream<T1, T2>::scientific;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f75() {
  return std::basic_ospanstream<T1, T2>::showbase;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f76() {
  return std::basic_ospanstream<T1, T2>::showpoint;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f77() {
  return std::basic_ospanstream<T1, T2>::showpos;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f78() {
  return std::basic_ospanstream<T1, T2>::skipws;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f79() {
  return std::basic_ospanstream<T1, T2>::unitbuf;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f80() {
  return std::basic_ospanstream<T1, T2>::uppercase;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f81() {
  return std::basic_ospanstream<T1, T2>::adjustfield;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f82() {
  return std::basic_ospanstream<T1, T2>::basefield;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags f83() {
  return std::basic_ospanstream<T1, T2>::floatfield;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate f84() {
  return std::basic_ospanstream<T1, T2>::badbit;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate f85() {
  return std::basic_ospanstream<T1, T2>::eofbit;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate f86() {
  return std::basic_ospanstream<T1, T2>::failbit;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::iostate f87() {
  return std::basic_ospanstream<T1, T2>::goodbit;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f88() {
  return std::basic_ospanstream<T1, T2>::app;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f89() {
  return std::basic_ospanstream<T1, T2>::ate;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f90() {
  return std::basic_ospanstream<T1, T2>::binary;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f91() {
  return std::basic_ospanstream<T1, T2>::in;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f92() {
  return std::basic_ospanstream<T1, T2>::out;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f93() {
  return std::basic_ospanstream<T1, T2>::trunc;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::openmode f94() {
  return std::basic_ospanstream<T1, T2>::noreplace;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::seekdir f95() {
  return std::basic_ospanstream<T1, T2>::beg;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::seekdir f96() {
  return std::basic_ospanstream<T1, T2>::cur;
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::seekdir f97() {
  return std::basic_ospanstream<T1, T2>::end;
}
