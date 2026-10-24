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
  return std::basic_spanstream<T1, T2>(a0, a1);
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
  return a0.span(a1);
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
  return a0.get(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f29(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f30(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f31(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f32(std::basic_spanstream<T1, T2> &a0, T1 *a1,
                                   std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, a3);
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
  return a0.ignore(a1, a2);
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
  return a0.putback(a1);
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
  return a0.seekg(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f45(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::off_type a1,
    typename std::basic_spanstream<T1, T2>::seekdir a2) {
  return a0.seekg(a1, a2);
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
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f50(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
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
  return a0.exceptions(a1);
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
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f61(std::basic_spanstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f62(const std::basic_spanstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(a1, a2);
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
  return a0.flags(a1);
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f67(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f68(std::basic_spanstream<T1, T2> &a0,
    typename std::basic_spanstream<T1, T2>::fmtflags a1,
    typename std::basic_spanstream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f69(std::basic_spanstream<T1, T2> &a0,
         typename std::basic_spanstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
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
std::basic_spanstream<T1, T2> &
f79(std::basic_spanstream<T1, T2> &a0,
    std::basic_spanstream<T1, T2> &(*a1)(std::basic_spanstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f80(std::basic_spanstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f81(std::basic_spanstream<T1, T2> &a0,
                                   std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f82(std::basic_spanstream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f83(std::basic_spanstream<T1, T2> &a0,
                                   unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f84(std::basic_spanstream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f85(std::basic_spanstream<T1, T2> &a0,
                                   short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f86(std::basic_spanstream<T1, T2> &a0,
                                   unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f87(std::basic_spanstream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f88(std::basic_spanstream<T1, T2> &a0,
                                   unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f89(std::basic_spanstream<T1, T2> &a0,
                                   long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f90(std::basic_spanstream<T1, T2> &a0,
                                   unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f91(std::basic_spanstream<T1, T2> &a0,
                                   double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f92(std::basic_spanstream<T1, T2> &a0,
                                   float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f93(std::basic_spanstream<T1, T2> &a0,
                                   long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f94(std::basic_spanstream<T1, T2> &a0,
                                   const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f95(std::basic_spanstream<T1, T2> &a0,
                                   std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f96(std::basic_spanstream<T1, T2> &a0,
                                   const volatile void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f97(std::basic_spanstream<T1, T2> &a0,
                                   std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f98(std::basic_spanstream<T1, T2> &a0, T1 a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f99(std::basic_spanstream<T1, T2> &a0,
                                   T1 const *a1, std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &f100(std::basic_spanstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_spanstream<T1, T2>::pos_type
f101(std::basic_spanstream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f102(std::basic_spanstream<T1, T2> &a0,
     typename std::basic_spanstream<T1, T2>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_spanstream<T1, T2> &
f103(std::basic_spanstream<T1, T2> &a0,
     typename std::basic_spanstream<T1, T2>::off_type a1,
     typename std::basic_spanstream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
}
