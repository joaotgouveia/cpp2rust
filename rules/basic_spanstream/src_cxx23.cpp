#include <algorithm>
#include <ios>
#include <iosfwd>
#include <span>
#include <spanstream>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_spanstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_spanstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_spanstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_spanstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_spanstream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_spanstream<T1, T2>::traits_type;

template <typename T1, typename T2>
std::basic_spanstream<T1, T2>
f1(std::span<T1> a0, typename std::basic_spanstream<T1, T2>::openmode a1) {
  return std::basic_spanstream<T1, T2>(
      std::forward<std::span<T1>>(a0),
      std::forward<typename std::basic_spanstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> f2(std::basic_spanstream<T1, T2> &&a0) {
  return std::basic_spanstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f3(std::basic_spanstream<T1, T2> &a0,
                                  std::basic_spanstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f4(std::basic_spanstream<T1, T2> &a0, std::basic_spanstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> *f5(const std::basic_spanstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::span<T1> f6(const std::basic_spanstream<T1, T2> &a0) {
  return a0.span();
}

template <typename T1, typename T2>
void f7(std::basic_spanstream<T1, T2> &a0, std::span<T1> a1) {
  return a0.span(std::forward<std::span<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f8(std::basic_spanstream<T1, T2> &a0,
   std::basic_spanstream<T1, T2> &(*a1)(std::basic_spanstream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f9(std::basic_spanstream<T1, T2> &a0,
   std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f10(std::basic_spanstream<T1, T2> &a0,
                                   std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f11(std::basic_spanstream<T1, T2> &a0,
                                   bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f12(std::basic_spanstream<T1, T2> &a0,
                                   short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f13(std::basic_spanstream<T1, T2> &a0,
                                   unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f14(std::basic_spanstream<T1, T2> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f15(std::basic_spanstream<T1, T2> &a0,
                                   unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f16(std::basic_spanstream<T1, T2> &a0,
                                   long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f17(std::basic_spanstream<T1, T2> &a0,
                                   unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f18(std::basic_spanstream<T1, T2> &a0,
                                   long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f19(std::basic_spanstream<T1, T2> &a0,
                                   unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f20(std::basic_spanstream<T1, T2> &a0,
                                   float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f21(std::basic_spanstream<T1, T2> &a0,
                                   double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f22(std::basic_spanstream<T1, T2> &a0,
                                   long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f23(std::basic_spanstream<T1, T2> &a0,
                                   void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f24(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::streamsize f25(const std::basic_spanstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::int_type
f26(std::basic_spanstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f27(std::basic_spanstream<T1, T2> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f28(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f29(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f30(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f31(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f32(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f33(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f34(std::basic_spanstream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_spanstream<T1, T2>::int_type a2) {
  return a0.ignore(
      a1, std::forward<typename std::basic_spanstream<T1, T2>::int_type>(a2));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f35(std::basic_spanstream<T1, T2> &a0,
                                   std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f36(std::basic_spanstream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::int_type
f37(std::basic_spanstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f38(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f39(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f40(std::basic_spanstream<T1, T2> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f41(std::basic_spanstream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2> int f42(std::basic_spanstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::pos_type
f43(std::basic_spanstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f44(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::pos_type a1) {
  return a0.seekg(
      std::forward<typename std::basic_spanstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f45(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::off_type a1,
    typename std::basic_spanstream<T1, T2>::seekdir a2) {
  return a0.seekg(
      std::forward<typename std::basic_spanstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_spanstream<T1, T2>::seekdir>(a2));
}

template <typename T1, typename T2>
bool f46(const std::basic_spanstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f47(const std::basic_spanstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate
f48(const std::basic_spanstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f49(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_spanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f50(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_spanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
bool f51(const std::basic_spanstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f52(const std::basic_spanstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f53(const std::basic_spanstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f54(const std::basic_spanstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate
f55(const std::basic_spanstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f56(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_spanstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f57(const std::basic_spanstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f58(std::basic_spanstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f59(const std::basic_spanstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f60(std::basic_spanstream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f61(std::basic_spanstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f62(const std::basic_spanstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f63(const std::basic_spanstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f64(std::basic_spanstream<T1, T2> &a0,
         void (*a1)(typename std::basic_spanstream<T1, T2>::event,
                    std::basic_spanstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f65(const std::basic_spanstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f66(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_spanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f67(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_spanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f68(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::fmtflags a1,
    typename std::basic_spanstream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_spanstream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_spanstream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f69(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_spanstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f70(const std::basic_spanstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f71(std::basic_spanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f72(const std::basic_spanstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f73(std::basic_spanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f74(bool a0) {
  return std::basic_spanstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f75(const std::basic_spanstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f76() {
  return std::basic_spanstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f77(std::basic_spanstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f78(std::basic_spanstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f79() {
  return std::basic_spanstream<T1, T2>::boolalpha;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f80() {
  return std::basic_spanstream<T1, T2>::dec;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f81() {
  return std::basic_spanstream<T1, T2>::fixed;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f82() {
  return std::basic_spanstream<T1, T2>::hex;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f83() {
  return std::basic_spanstream<T1, T2>::internal;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f84() {
  return std::basic_spanstream<T1, T2>::left;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f85() {
  return std::basic_spanstream<T1, T2>::oct;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f86() {
  return std::basic_spanstream<T1, T2>::right;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f87() {
  return std::basic_spanstream<T1, T2>::scientific;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f88() {
  return std::basic_spanstream<T1, T2>::showbase;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f89() {
  return std::basic_spanstream<T1, T2>::showpoint;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f90() {
  return std::basic_spanstream<T1, T2>::showpos;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f91() {
  return std::basic_spanstream<T1, T2>::skipws;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f92() {
  return std::basic_spanstream<T1, T2>::unitbuf;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f93() {
  return std::basic_spanstream<T1, T2>::uppercase;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f94() {
  return std::basic_spanstream<T1, T2>::adjustfield;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f95() {
  return std::basic_spanstream<T1, T2>::basefield;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::fmtflags f96() {
  return std::basic_spanstream<T1, T2>::floatfield;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate f97() {
  return std::basic_spanstream<T1, T2>::badbit;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate f98() {
  return std::basic_spanstream<T1, T2>::eofbit;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate f99() {
  return std::basic_spanstream<T1, T2>::failbit;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::iostate f100() {
  return std::basic_spanstream<T1, T2>::goodbit;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f101() {
  return std::basic_spanstream<T1, T2>::app;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f102() {
  return std::basic_spanstream<T1, T2>::ate;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f103() {
  return std::basic_spanstream<T1, T2>::binary;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f104() {
  return std::basic_spanstream<T1, T2>::in;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f105() {
  return std::basic_spanstream<T1, T2>::out;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f106() {
  return std::basic_spanstream<T1, T2>::trunc;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::openmode f107() {
  return std::basic_spanstream<T1, T2>::noreplace;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::seekdir f108() {
  return std::basic_spanstream<T1, T2>::beg;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::seekdir f109() {
  return std::basic_spanstream<T1, T2>::cur;
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::seekdir f110() {
  return std::basic_spanstream<T1, T2>::end;
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f111(std::basic_spanstream<T1, T2> &a0,
     std::basic_spanstream<T1, T2> &(*a1)(std::basic_spanstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f112(std::basic_spanstream<T1, T2> &a0,
     std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f113(std::basic_spanstream<T1, T2> &a0,
                                    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f114(std::basic_spanstream<T1, T2> &a0,
                                    long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f115(std::basic_spanstream<T1, T2> &a0,
                                    unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f116(std::basic_spanstream<T1, T2> &a0,
                                    bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f117(std::basic_spanstream<T1, T2> &a0,
                                    short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f118(std::basic_spanstream<T1, T2> &a0,
                                    unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f119(std::basic_spanstream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f120(std::basic_spanstream<T1, T2> &a0,
                                    unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f121(std::basic_spanstream<T1, T2> &a0,
                                    long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f122(std::basic_spanstream<T1, T2> &a0,
                                    unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f123(std::basic_spanstream<T1, T2> &a0,
                                    double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f124(std::basic_spanstream<T1, T2> &a0,
                                    float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f125(std::basic_spanstream<T1, T2> &a0,
                                    long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f126(std::basic_spanstream<T1, T2> &a0,
                                    const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f127(std::basic_spanstream<T1, T2> &a0,
                                    std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f128(std::basic_spanstream<T1, T2> &a0,
                                    const volatile void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f129(std::basic_spanstream<T1, T2> &a0,
                                    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f130(std::basic_spanstream<T1, T2> &a0, T1 a1) {
  return a0.put(std::move(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f131(std::basic_spanstream<T1, T2> &a0,
                                    T1 const *a1, std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f132(std::basic_spanstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::pos_type
f133(std::basic_spanstream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f134(std::basic_spanstream<T1, T2> &a0,
     typename std::basic_spanstream<T1, T2>::pos_type a1) {
  return a0.seekp(
      std::forward<typename std::basic_spanstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f135(std::basic_spanstream<T1, T2> &a0,
     typename std::basic_spanstream<T1, T2>::off_type a1,
     typename std::basic_spanstream<T1, T2>::seekdir a2) {
  return a0.seekp(
      std::forward<typename std::basic_spanstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_spanstream<T1, T2>::seekdir>(a2));
}
