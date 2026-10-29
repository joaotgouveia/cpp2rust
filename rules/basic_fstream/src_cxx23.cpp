#include <algorithm>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_fstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_fstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_fstream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_fstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::basic_fstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t6 = typename std::basic_fstream<T1, T2>::off_type;

template <typename T1, typename T2> std::basic_fstream<T1, T2> f1() {
  return std::basic_fstream<T1, T2>();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2>
f2(const char *a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(
      a0, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2>
f3(const std::string &a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(
      a0, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> f4(std::basic_fstream<T1, T2> &&a0) {
  return std::basic_fstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f5(std::basic_fstream<T1, T2> &a0,
                               std::basic_fstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f6(std::basic_fstream<T1, T2> &a0, std::basic_fstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_filebuf<T1, T2> *f7(const std::basic_fstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2> bool f8(std::basic_fstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
bool f9(const std::basic_fstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
void f10(std::basic_fstream<T1, T2> &a0, const char *a1,
         typename std::basic_fstream<T1, T2>::openmode a2) {
  return a0.open(
      a1, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2>
void f11(std::basic_fstream<T1, T2> &a0, const std::string &a1,
         typename std::basic_fstream<T1, T2>::openmode a2) {
  return a0.open(
      a1, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2> void f12(std::basic_fstream<T1, T2> &a0) {
  return a0.close();
}

template <typename T1, typename T2, typename T3>
std::basic_fstream<T1, T2> f13(const T3 &a0,
                               typename std::basic_fstream<T1, T2>::openmode a1)
  requires(!(std::is_same<T3, std::string>::value))
{
  return std::basic_fstream<T1, T2>(
      a0, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2, typename T3>
void f14(std::basic_fstream<T1, T2> &a0, const T3 &a1,
         typename std::basic_fstream<T1, T2>::openmode a2)
  requires(!(std::is_same<T3, std::string>::value))
{
  return a0.template open<T3>(
      a1, std::forward<typename std::basic_fstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f15(std::basic_fstream<T1, T2> &a0,
    std::basic_fstream<T1, T2> &(*a1)(std::basic_fstream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f16(std::basic_fstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f17(std::basic_fstream<T1, T2> &a0,
                                std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f18(std::basic_fstream<T1, T2> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f19(std::basic_fstream<T1, T2> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f20(std::basic_fstream<T1, T2> &a0,
                                unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f21(std::basic_fstream<T1, T2> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f22(std::basic_fstream<T1, T2> &a0,
                                unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f23(std::basic_fstream<T1, T2> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f24(std::basic_fstream<T1, T2> &a0,
                                unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f25(std::basic_fstream<T1, T2> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f26(std::basic_fstream<T1, T2> &a0,
                                unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f27(std::basic_fstream<T1, T2> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f28(std::basic_fstream<T1, T2> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f29(std::basic_fstream<T1, T2> &a0,
                                long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f30(std::basic_fstream<T1, T2> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f31(std::basic_fstream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::streamsize f32(const std::basic_fstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::int_type
f33(std::basic_fstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f34(std::basic_fstream<T1, T2> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f35(std::basic_fstream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f36(std::basic_fstream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f37(std::basic_fstream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f38(std::basic_fstream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f39(std::basic_fstream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f40(std::basic_fstream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f41(std::basic_fstream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_fstream<T1, T2>::int_type a2) {
  return a0.ignore(
      a1, std::forward<typename std::basic_fstream<T1, T2>::int_type>(a2));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f42(std::basic_fstream<T1, T2> &a0,
                                std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f43(std::basic_fstream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::int_type
f44(std::basic_fstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f45(std::basic_fstream<T1, T2> &a0, T1 *a1,
                                std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f46(std::basic_fstream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f47(std::basic_fstream<T1, T2> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f48(std::basic_fstream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2> int f49(std::basic_fstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::pos_type
f50(std::basic_fstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f51(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::pos_type a1) {
  return a0.seekg(
      std::forward<typename std::basic_fstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f52(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::off_type a1,
    typename std::basic_fstream<T1, T2>::seekdir a2) {
  return a0.seekg(
      std::forward<typename std::basic_fstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_fstream<T1, T2>::seekdir>(a2));
}

template <typename T1, typename T2>
bool f53(const std::basic_fstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f54(const std::basic_fstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::iostate
f55(const std::basic_fstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f56(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_fstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f57(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_fstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
bool f58(const std::basic_fstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f59(const std::basic_fstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f60(const std::basic_fstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f61(const std::basic_fstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::iostate
f62(const std::basic_fstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f63(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_fstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f64(const std::basic_fstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f65(std::basic_fstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f66(const std::basic_fstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f67(std::basic_fstream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f68(std::basic_fstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f69(const std::basic_fstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f70(const std::basic_fstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f71(std::basic_fstream<T1, T2> &a0,
         void (*a1)(typename std::basic_fstream<T1, T2>::event,
                    std::basic_fstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f72(const std::basic_fstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f73(std::basic_fstream<T1, T2> &a0,
                            typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_fstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f74(std::basic_fstream<T1, T2> &a0,
                            typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_fstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f75(std::basic_fstream<T1, T2> &a0,
                            typename std::basic_fstream<T1, T2>::fmtflags a1,
                            typename std::basic_fstream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_fstream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_fstream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f76(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_fstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f77(const std::basic_fstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f78(std::basic_fstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f79(const std::basic_fstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f80(std::basic_fstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f81(bool a0) {
  return std::basic_fstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f82(const std::basic_fstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f83() {
  return std::basic_fstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f84(std::basic_fstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f85(std::basic_fstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f86(std::basic_fstream<T1, T2> &a0,
    std::basic_fstream<T1, T2> &(*a1)(std::basic_fstream<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f87(std::basic_fstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f88(std::basic_fstream<T1, T2> &a0,
                                std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f89(std::basic_fstream<T1, T2> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f90(std::basic_fstream<T1, T2> &a0,
                                unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f91(std::basic_fstream<T1, T2> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f92(std::basic_fstream<T1, T2> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f93(std::basic_fstream<T1, T2> &a0,
                                unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f94(std::basic_fstream<T1, T2> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f95(std::basic_fstream<T1, T2> &a0,
                                unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f96(std::basic_fstream<T1, T2> &a0, long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f97(std::basic_fstream<T1, T2> &a0,
                                unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f98(std::basic_fstream<T1, T2> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f99(std::basic_fstream<T1, T2> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f100(std::basic_fstream<T1, T2> &a0,
                                 long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f101(std::basic_fstream<T1, T2> &a0,
                                 const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f102(std::basic_fstream<T1, T2> &a0,
                                 std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f103(std::basic_fstream<T1, T2> &a0,
                                 const volatile void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f104(std::basic_fstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f105(std::basic_fstream<T1, T2> &a0, T1 a1) {
  return a0.put(std::move(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f106(std::basic_fstream<T1, T2> &a0, T1 const *a1,
                                 std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f107(std::basic_fstream<T1, T2> &a0) {
  return a0.flush();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::pos_type
f108(std::basic_fstream<T1, T2> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f109(std::basic_fstream<T1, T2> &a0,
     typename std::basic_fstream<T1, T2>::pos_type a1) {
  return a0.seekp(
      std::forward<typename std::basic_fstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &
f110(std::basic_fstream<T1, T2> &a0,
     typename std::basic_fstream<T1, T2>::off_type a1,
     typename std::basic_fstream<T1, T2>::seekdir a2) {
  return a0.seekp(
      std::forward<typename std::basic_fstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_fstream<T1, T2>::seekdir>(a2));
}
