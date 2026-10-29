#include <ios>
#include <iosfwd>
#include <istream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_istream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_istream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_istream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_istream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_istream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_istream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t7 = typename std::basic_istream<T1, T2>::sentry;

template <typename T1, typename T2>
std::basic_istream<T1, T2> f1(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_istream<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f2(std::basic_istream<T1, T2> &a0,
   std::basic_istream<T1, T2> &(*a1)(std::basic_istream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f3(std::basic_istream<T1, T2> &a0,
   std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f4(std::basic_istream<T1, T2> &a0,
                               std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f5(std::basic_istream<T1, T2> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f6(std::basic_istream<T1, T2> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f7(std::basic_istream<T1, T2> &a0,
                               unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f8(std::basic_istream<T1, T2> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f9(std::basic_istream<T1, T2> &a0,
                               unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f10(std::basic_istream<T1, T2> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f11(std::basic_istream<T1, T2> &a0,
                                unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f12(std::basic_istream<T1, T2> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f13(std::basic_istream<T1, T2> &a0,
                                unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f15(std::basic_istream<T1, T2> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f16(std::basic_istream<T1, T2> &a0,
                                long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::streamsize f19(const std::basic_istream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::int_type
f20(std::basic_istream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f21(std::basic_istream<T1, T2> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f22(std::basic_istream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f23(std::basic_istream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f24(std::basic_istream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f25(std::basic_istream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f26(std::basic_istream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f27(std::basic_istream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f28(std::basic_istream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_istream<T1, T2>::int_type a2) {
  return a0.ignore(
      a1, std::forward<typename std::basic_istream<T1, T2>::int_type>(a2));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f29(std::basic_istream<T1, T2> &a0,
                                std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f30(std::basic_istream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::int_type
f31(std::basic_istream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f32(std::basic_istream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f33(std::basic_istream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f34(std::basic_istream<T1, T2> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f35(std::basic_istream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2> int f36(std::basic_istream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::pos_type
f37(std::basic_istream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f38(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::pos_type a1) {
  return a0.seekg(
      std::forward<typename std::basic_istream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f39(std::basic_istream<T1, T2> &a0,
    typename std::basic_istream<T1, T2>::off_type a1,
    typename std::basic_istream<T1, T2>::seekdir a2) {
  return a0.seekg(
      std::forward<typename std::basic_istream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_istream<T1, T2>::seekdir>(a2));
}

template <typename T1, typename T2>
bool f40(const std::basic_istream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f41(const std::basic_istream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f42(const std::basic_istream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f43(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_istream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f44(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_istream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
bool f45(const std::basic_istream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f46(const std::basic_istream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f47(const std::basic_istream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f48(const std::basic_istream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::iostate
f49(const std::basic_istream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f50(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_istream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f51(const std::basic_istream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f52(std::basic_istream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f53(const std::basic_istream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f54(std::basic_istream<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2>
T1 f55(const std::basic_istream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f56(std::basic_istream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f57(std::basic_istream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f58(const std::basic_istream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f59(const std::basic_istream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f60(std::basic_istream<T1, T2> &a0,
         void (*a1)(typename std::basic_istream<T1, T2>::event,
                    std::basic_istream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f61(const std::basic_istream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f62(std::basic_istream<T1, T2> &a0,
                            typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_istream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f63(std::basic_istream<T1, T2> &a0,
                            typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_istream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f64(std::basic_istream<T1, T2> &a0,
                            typename std::basic_istream<T1, T2>::fmtflags a1,
                            typename std::basic_istream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_istream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_istream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f65(std::basic_istream<T1, T2> &a0,
         typename std::basic_istream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_istream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f66(const std::basic_istream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f67(std::basic_istream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f68(const std::basic_istream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f69(std::basic_istream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f70(bool a0) {
  return std::basic_istream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f71(const std::basic_istream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f72() {
  return std::basic_istream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f73(std::basic_istream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f74(std::basic_istream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
using t8 = typename std::basic_istream<T1, T2>::sentry::traits_type;

template <typename T1, typename T2>
typename std::basic_istream<T1, T2>::sentry f75(std::basic_istream<T1, T2> &a0,
                                                bool a1) {
  return typename std::basic_istream<T1, T2>::sentry(a0, a1);
}

template <typename T1, typename T2>
bool f76(const typename std::basic_istream<T1, T2>::sentry &a0) {
  return static_cast<bool>(a0);
}
