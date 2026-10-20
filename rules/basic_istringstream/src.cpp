#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_istringstream<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_istringstream<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_istringstream<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_istringstream<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_istringstream<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_istringstream<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_istringstream<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> f1() {
  return std::basic_istringstream<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f2(typename std::basic_istringstream<T1, T2, T3>::openmode a0) {
  return std::basic_istringstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f3(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f4(std::basic_istringstream<T1, T2, T3> &&a0) {
  return std::basic_istringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f5(std::basic_istringstream<T1, T2, T3> &a0,
   std::basic_istringstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f6(std::basic_istringstream<T1, T2, T3> &a0,
        std::basic_istringstream<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> *
f7(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f8(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f9(std::basic_istringstream<T1, T2, T3> &a0,
        const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f10(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_istringstream<T1, T2, T3> &(*a1)(
        std::basic_istringstream<T1, T2, T3> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f11(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f12(std::basic_istringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f13(std::basic_istringstream<T1, T2, T3> &a0, bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f14(std::basic_istringstream<T1, T2, T3> &a0, short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f15(std::basic_istringstream<T1, T2, T3> &a0, unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f16(std::basic_istringstream<T1, T2, T3> &a0, int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f17(std::basic_istringstream<T1, T2, T3> &a0, unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f18(std::basic_istringstream<T1, T2, T3> &a0, long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f19(std::basic_istringstream<T1, T2, T3> &a0, unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f20(std::basic_istringstream<T1, T2, T3> &a0, long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f21(std::basic_istringstream<T1, T2, T3> &a0, unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f22(std::basic_istringstream<T1, T2, T3> &a0, float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f23(std::basic_istringstream<T1, T2, T3> &a0, double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f24(std::basic_istringstream<T1, T2, T3> &a0, long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f25(std::basic_istringstream<T1, T2, T3> &a0, void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f26(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f27(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::int_type
f28(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.get();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f29(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f30(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2,
    typename std::basic_istringstream<T1, T2, T3>::char_type a3) {
  return a0.get(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f31(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f32(std::basic_istringstream<T1, T2, T3> &a0, std::basic_streambuf<T1, T2> &a1,
    typename std::basic_istringstream<T1, T2, T3>::char_type a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f33(std::basic_istringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f34(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2,
    typename std::basic_istringstream<T1, T2, T3>::char_type a3) {
  return a0.getline(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f35(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f36(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1,
    typename std::basic_istringstream<T1, T2, T3>::int_type a2) {
  return a0.ignore(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f37(std::basic_istringstream<T1, T2, T3> &a0, std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f38(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::int_type
f39(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.peek();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f40(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::streamsize
f41(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f42(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type a1) {
  return a0.putback(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f43(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.unget();
}

template <typename T1, typename T2, typename T3>
int f44(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.sync();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::pos_type
f45(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f46(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekg(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f47(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::off_type a1,
    typename std::basic_istringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekg(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f48(const std::basic_istringstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f49(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f50(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f51(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f52(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
bool f53(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f54(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f55(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f56(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f57(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f58(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *
f59(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f60(std::basic_istringstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f61(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f62(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f63(std::basic_istringstream<T1, T2, T3> &a0,
                const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f64(const std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f65(const std::basic_istringstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f66(std::basic_istringstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_istringstream<T1, T2, T3>::event,
                    std::basic_istringstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f67(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f68(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f69(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f70(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f71(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f72(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f73(std::basic_istringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f74(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f75(std::basic_istringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f76(bool a0) {
  return std::basic_istringstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f77(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f78() {
  return std::basic_istringstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f79(std::basic_istringstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f80(std::basic_istringstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}
