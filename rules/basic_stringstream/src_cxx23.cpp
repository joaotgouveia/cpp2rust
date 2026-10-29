#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_stringstream<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_stringstream<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_stringstream<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_stringstream<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_stringstream<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_stringstream<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_stringstream<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> f1() {
  return std::basic_stringstream<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f2(typename std::basic_stringstream<T1, T2, T3>::openmode a0) {
  return std::basic_stringstream<T1, T2, T3>(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f3(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(
      a0,
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f4(std::basic_stringstream<T1, T2, T3> &&a0) {
  return std::basic_stringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f5(typename std::basic_stringstream<T1, T2, T3>::openmode a0, const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a0),
      a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f6(std::basic_string<T1, T2, T3> &&a0,
   typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(
      std::move(a0),
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f7(std::basic_stringstream<T1, T2, T3> &a0,
   std::basic_stringstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f8(std::basic_stringstream<T1, T2, T3> &a0,
        std::basic_stringstream<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> *
f9(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f10(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f11(std::basic_stringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2>
f12(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f13(std::basic_stringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
void f14(std::basic_stringstream<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3> f15(const std::basic_string<T1, T2, T4> &a0,
                                        const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f16(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_stringstream<T1, T2, T3>::openmode a1, const T3 &a2) {
  return std::basic_stringstream<T1, T2, T3>(
      a0,
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a1),
      a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f17(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_stringstream<T1, T2, T3>::openmode a1)
  requires(!(std::is_same<T4, T3>::value))
{
  return std::basic_stringstream<T1, T2, T3>(
      a0,
      std::forward<typename std::basic_stringstream<T1, T2, T3>::openmode>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4> f18(const std::basic_stringstream<T1, T2, T3> &a0,
                                  const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f19(std::basic_stringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1)
  requires(((!std::is_same_v<T4, T3>)) && (!(std::is_same<T4, T3>::value)))
{
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f20(std::basic_stringstream<T1, T2, T3> &a0,
    std::basic_stringstream<T1, T2, T3> &(*a1)(
        std::basic_stringstream<T1, T2, T3> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f21(std::basic_stringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f22(std::basic_stringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f23(std::basic_stringstream<T1, T2, T3> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f24(std::basic_stringstream<T1, T2, T3> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f25(std::basic_stringstream<T1, T2, T3> &a0, unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f26(std::basic_stringstream<T1, T2, T3> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f27(std::basic_stringstream<T1, T2, T3> &a0, unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f28(std::basic_stringstream<T1, T2, T3> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f29(std::basic_stringstream<T1, T2, T3> &a0, unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f30(std::basic_stringstream<T1, T2, T3> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f31(std::basic_stringstream<T1, T2, T3> &a0, unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f32(std::basic_stringstream<T1, T2, T3> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f33(std::basic_stringstream<T1, T2, T3> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f34(std::basic_stringstream<T1, T2, T3> &a0, long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f35(std::basic_stringstream<T1, T2, T3> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f36(std::basic_stringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f37(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::int_type
f38(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.get();
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f39(std::basic_stringstream<T1, T2, T3> &a0, T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f40(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2,
    T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f41(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f42(std::basic_stringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> &a1,
    T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f43(std::basic_stringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f44(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2,
    T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f45(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f46(std::basic_stringstream<T1, T2, T3> &a0, std::streamsize a1,
    typename std::basic_stringstream<T1, T2, T3>::int_type a2) {
  return a0.ignore(
      a1,
      std::forward<typename std::basic_stringstream<T1, T2, T3>::int_type>(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f47(std::basic_stringstream<T1, T2, T3> &a0, std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f48(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::int_type
f49(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.peek();
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f50(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1, std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::streamsize f51(std::basic_stringstream<T1, T2, T3> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f52(std::basic_stringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f53(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.unget();
}

template <typename T1, typename T2, typename T3>
int f54(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.sync();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::pos_type
f55(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f56(std::basic_stringstream<T1, T2, T3> &a0,
    typename std::basic_stringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekg(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::pos_type>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f57(std::basic_stringstream<T1, T2, T3> &a0,
    typename std::basic_stringstream<T1, T2, T3>::off_type a1,
    typename std::basic_stringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekg(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::off_type>(a1),
      std::forward<typename std::basic_stringstream<T1, T2, T3>::seekdir>(a2));
}

template <typename T1, typename T2, typename T3>
bool f58(const std::basic_stringstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f59(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::iostate
f60(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f61(std::basic_stringstream<T1, T2, T3> &a0,
         typename std::basic_stringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
void f62(std::basic_stringstream<T1, T2, T3> &a0,
         typename std::basic_stringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
bool f63(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f64(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f65(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f66(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::iostate
f67(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f68(std::basic_stringstream<T1, T2, T3> &a0,
         typename std::basic_stringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f69(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f70(std::basic_stringstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2, typename T3>
T1 f71(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
T1 f72(std::basic_stringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::locale f73(std::basic_stringstream<T1, T2, T3> &a0,
                const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f74(const std::basic_stringstream<T1, T2, T3> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f75(const std::basic_stringstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f76(std::basic_stringstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_stringstream<T1, T2, T3>::event,
                    std::basic_stringstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags f77(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f78(std::basic_stringstream<T1, T2, T3> &a0,
    typename std::basic_stringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f79(std::basic_stringstream<T1, T2, T3> &a0,
    typename std::basic_stringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f80(std::basic_stringstream<T1, T2, T3> &a0,
    typename std::basic_stringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_stringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::fmtflags>(a1),
      std::forward<typename std::basic_stringstream<T1, T2, T3>::fmtflags>(a2));
}

template <typename T1, typename T2, typename T3>
void f81(std::basic_stringstream<T1, T2, T3> &a0,
         typename std::basic_stringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::streamsize f82(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f83(std::basic_stringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f84(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f85(std::basic_stringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f86(bool a0) {
  return std::basic_stringstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f87(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f88() {
  return std::basic_stringstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f89(std::basic_stringstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f90(std::basic_stringstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f91(std::basic_stringstream<T1, T2, T3> &a0,
    std::basic_stringstream<T1, T2, T3> &(*a1)(
        std::basic_stringstream<T1, T2, T3> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f92(std::basic_stringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f93(std::basic_stringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f94(std::basic_stringstream<T1, T2, T3> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f95(std::basic_stringstream<T1, T2, T3> &a0, unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f96(std::basic_stringstream<T1, T2, T3> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f97(std::basic_stringstream<T1, T2, T3> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f98(std::basic_stringstream<T1, T2, T3> &a0, unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f99(std::basic_stringstream<T1, T2, T3> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f100(std::basic_stringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f101(std::basic_stringstream<T1, T2, T3> &a0, long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f102(std::basic_stringstream<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f103(std::basic_stringstream<T1, T2, T3> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f104(std::basic_stringstream<T1, T2, T3> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f105(std::basic_stringstream<T1, T2, T3> &a0, long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f106(std::basic_stringstream<T1, T2, T3> &a0, const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f107(std::basic_stringstream<T1, T2, T3> &a0, std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f108(std::basic_stringstream<T1, T2, T3> &a0, const volatile void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f109(std::basic_stringstream<T1, T2, T3> &a0,
     std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f110(std::basic_stringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.put(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f111(std::basic_stringstream<T1, T2, T3> &a0, T1 const *a1,
     std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f112(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.flush();
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::pos_type
f113(std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f114(std::basic_stringstream<T1, T2, T3> &a0,
     typename std::basic_stringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekp(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::pos_type>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f115(std::basic_stringstream<T1, T2, T3> &a0,
     typename std::basic_stringstream<T1, T2, T3>::off_type a1,
     typename std::basic_stringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekp(
      std::forward<typename std::basic_stringstream<T1, T2, T3>::off_type>(a1),
      std::forward<typename std::basic_stringstream<T1, T2, T3>::seekdir>(a2));
}
