#include <fstream>
#include <ios>
#include <iosfwd>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ifstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ifstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ifstream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ifstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ifstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ifstream<T1, T2>::off_type;

template <typename T1, typename T2> std::basic_ifstream<T1, T2> f1() {
  return std::basic_ifstream<T1, T2>();
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2>
f2(const char *a0, typename std::basic_ifstream<T1, T2>::openmode a1) {
  return std::basic_ifstream<T1, T2>(
      a0, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2>
f3(const std::string &a0, typename std::basic_ifstream<T1, T2>::openmode a1) {
  return std::basic_ifstream<T1, T2>(
      a0, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> f4(std::basic_ifstream<T1, T2> &&a0) {
  return std::basic_ifstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f5(std::basic_ifstream<T1, T2> &a0,
                                std::basic_ifstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f6(std::basic_ifstream<T1, T2> &a0, std::basic_ifstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_filebuf<T1, T2> *f7(const std::basic_ifstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2> bool f8(std::basic_ifstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
bool f9(const std::basic_ifstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
void f10(std::basic_ifstream<T1, T2> &a0, const char *a1,
         typename std::basic_ifstream<T1, T2>::openmode a2) {
  return a0.open(
      a1, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2>
void f11(std::basic_ifstream<T1, T2> &a0, const std::string &a1,
         typename std::basic_ifstream<T1, T2>::openmode a2) {
  return a0.open(
      a1, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2> void f12(std::basic_ifstream<T1, T2> &a0) {
  return a0.close();
}

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::enable_if_t<
        std::is_same_v<
            T3, decltype(std::declval<T3 &>().make_preferred().filename())>,
        T3>>
std::basic_ifstream<T1, T2>
f13(const T3 &a0, typename std::basic_ifstream<T1, T2>::openmode a1)
  requires(!(std::is_same<T3, std::string>::value))
{
  return std::basic_ifstream<T1, T2>(
      a0, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a1));
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<
    std::is_same_v<T3,
                   decltype(std::declval<T3 &>().make_preferred().filename())>,
    void>
f14(std::basic_ifstream<T1, T2> &a0, const T3 &a1,
    typename std::basic_ifstream<T1, T2>::openmode a2)
  requires(!(std::is_same<T3, std::string>::value))
{
  return a0.template open<T3>(
      a1, std::forward<typename std::basic_ifstream<T1, T2>::openmode>(a2));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &
f15(std::basic_ifstream<T1, T2> &a0,
    std::basic_ifstream<T1, T2> &(*a1)(std::basic_ifstream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &
f16(std::basic_ifstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f17(std::basic_ifstream<T1, T2> &a0,
                                 std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f18(std::basic_ifstream<T1, T2> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f19(std::basic_ifstream<T1, T2> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f20(std::basic_ifstream<T1, T2> &a0,
                                 unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f21(std::basic_ifstream<T1, T2> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f22(std::basic_ifstream<T1, T2> &a0,
                                 unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f23(std::basic_ifstream<T1, T2> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f24(std::basic_ifstream<T1, T2> &a0,
                                 unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f25(std::basic_ifstream<T1, T2> &a0,
                                 long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f26(std::basic_ifstream<T1, T2> &a0,
                                 unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f27(std::basic_ifstream<T1, T2> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f28(std::basic_ifstream<T1, T2> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f29(std::basic_ifstream<T1, T2> &a0,
                                 long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f30(std::basic_ifstream<T1, T2> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f31(std::basic_ifstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::streamsize f32(const std::basic_ifstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::int_type
f33(std::basic_ifstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f34(std::basic_ifstream<T1, T2> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f35(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f36(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f37(std::basic_ifstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f38(std::basic_ifstream<T1, T2> &a0,
                                 std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f39(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f40(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &
f41(std::basic_ifstream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_ifstream<T1, T2>::int_type a2) {
  return a0.ignore(
      a1, std::forward<typename std::basic_ifstream<T1, T2>::int_type>(a2));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f42(std::basic_ifstream<T1, T2> &a0,
                                 std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f43(std::basic_ifstream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::int_type
f44(std::basic_ifstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f45(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                                 std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f46(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f47(std::basic_ifstream<T1, T2> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f48(std::basic_ifstream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2> int f49(std::basic_ifstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::pos_type
f50(std::basic_ifstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &
f51(std::basic_ifstream<T1, T2> &a0,
    typename std::basic_ifstream<T1, T2>::pos_type a1) {
  return a0.seekg(
      std::forward<typename std::basic_ifstream<T1, T2>::pos_type>(a1));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &
f52(std::basic_ifstream<T1, T2> &a0,
    typename std::basic_ifstream<T1, T2>::off_type a1,
    typename std::basic_ifstream<T1, T2>::seekdir a2) {
  return a0.seekg(
      std::forward<typename std::basic_ifstream<T1, T2>::off_type>(a1),
      std::forward<typename std::basic_ifstream<T1, T2>::seekdir>(a2));
}

template <typename T1, typename T2>
bool f53(const std::basic_ifstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f54(const std::basic_ifstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate
f55(const std::basic_ifstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f56(std::basic_ifstream<T1, T2> &a0,
         typename std::basic_ifstream<T1, T2>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_ifstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f57(std::basic_ifstream<T1, T2> &a0,
         typename std::basic_ifstream<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_ifstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
bool f58(const std::basic_ifstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f59(const std::basic_ifstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f60(const std::basic_ifstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f61(const std::basic_ifstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate
f62(const std::basic_ifstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f63(std::basic_ifstream<T1, T2> &a0,
         typename std::basic_ifstream<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_ifstream<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f64(const std::basic_ifstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f65(std::basic_ifstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f66(const std::basic_ifstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f67(std::basic_ifstream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f68(std::basic_ifstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f69(const std::basic_ifstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f70(const std::basic_ifstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f71(std::basic_ifstream<T1, T2> &a0,
         void (*a1)(typename std::basic_ifstream<T1, T2>::event,
                    std::basic_ifstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f72(const std::basic_ifstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f73(std::basic_ifstream<T1, T2> &a0,
                            typename std::basic_ifstream<T1, T2>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_ifstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f74(std::basic_ifstream<T1, T2> &a0,
                            typename std::basic_ifstream<T1, T2>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_ifstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f75(std::basic_ifstream<T1, T2> &a0,
                            typename std::basic_ifstream<T1, T2>::fmtflags a1,
                            typename std::basic_ifstream<T1, T2>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_ifstream<T1, T2>::fmtflags>(a1),
      std::forward<typename std::basic_ifstream<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f76(std::basic_ifstream<T1, T2> &a0,
         typename std::basic_ifstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_ifstream<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f77(const std::basic_ifstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f78(std::basic_ifstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f79(const std::basic_ifstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f80(std::basic_ifstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f81(bool a0) {
  return std::basic_ifstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f82(const std::basic_ifstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f83() {
  return std::basic_ifstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f84(std::basic_ifstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f85(std::basic_ifstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f86() {
  return std::basic_ifstream<T1, T2>::boolalpha;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f87() {
  return std::basic_ifstream<T1, T2>::dec;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f88() {
  return std::basic_ifstream<T1, T2>::fixed;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f89() {
  return std::basic_ifstream<T1, T2>::hex;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f90() {
  return std::basic_ifstream<T1, T2>::internal;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f91() {
  return std::basic_ifstream<T1, T2>::left;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f92() {
  return std::basic_ifstream<T1, T2>::oct;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f93() {
  return std::basic_ifstream<T1, T2>::right;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f94() {
  return std::basic_ifstream<T1, T2>::scientific;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f95() {
  return std::basic_ifstream<T1, T2>::showbase;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f96() {
  return std::basic_ifstream<T1, T2>::showpoint;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f97() {
  return std::basic_ifstream<T1, T2>::showpos;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f98() {
  return std::basic_ifstream<T1, T2>::skipws;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f99() {
  return std::basic_ifstream<T1, T2>::unitbuf;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f100() {
  return std::basic_ifstream<T1, T2>::uppercase;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f101() {
  return std::basic_ifstream<T1, T2>::adjustfield;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f102() {
  return std::basic_ifstream<T1, T2>::basefield;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::fmtflags f103() {
  return std::basic_ifstream<T1, T2>::floatfield;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate f104() {
  return std::basic_ifstream<T1, T2>::badbit;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate f105() {
  return std::basic_ifstream<T1, T2>::eofbit;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate f106() {
  return std::basic_ifstream<T1, T2>::failbit;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::iostate f107() {
  return std::basic_ifstream<T1, T2>::goodbit;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f108() {
  return std::basic_ifstream<T1, T2>::app;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f109() {
  return std::basic_ifstream<T1, T2>::ate;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f110() {
  return std::basic_ifstream<T1, T2>::binary;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f111() {
  return std::basic_ifstream<T1, T2>::in;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f112() {
  return std::basic_ifstream<T1, T2>::out;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f113() {
  return std::basic_ifstream<T1, T2>::trunc;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::openmode f114() {
  return std::basic_ifstream<T1, T2>::noreplace;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::seekdir f115() {
  return std::basic_ifstream<T1, T2>::beg;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::seekdir f116() {
  return std::basic_ifstream<T1, T2>::cur;
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::seekdir f117() {
  return std::basic_ifstream<T1, T2>::end;
}
