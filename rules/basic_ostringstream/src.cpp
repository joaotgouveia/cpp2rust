#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_ostringstream<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_ostringstream<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_ostringstream<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_ostringstream<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_ostringstream<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_ostringstream<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_ostringstream<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> f1() {
  return std::basic_ostringstream<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f2(typename std::basic_ostringstream<T1, T2, T3>::openmode a0) {
  return std::basic_ostringstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f3(const std::basic_string<T1, T2, T3> &a0,
   typename std::basic_ostringstream<T1, T2, T3>::openmode a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f4(std::basic_ostringstream<T1, T2, T3> &&a0) {
  return std::basic_ostringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f5(std::basic_ostringstream<T1, T2, T3> &a0,
   std::basic_ostringstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f6(std::basic_ostringstream<T1, T2, T3> &a0,
        std::basic_ostringstream<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> *
f7(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f8(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f9(std::basic_ostringstream<T1, T2, T3> &a0,
        const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f10(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_ostringstream<T1, T2, T3> &(*a1)(
        std::basic_ostringstream<T1, T2, T3> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f11(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f12(std::basic_ostringstream<T1, T2, T3> &a0,
    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f13(std::basic_ostringstream<T1, T2, T3> &a0, long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f14(std::basic_ostringstream<T1, T2, T3> &a0, unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f15(std::basic_ostringstream<T1, T2, T3> &a0, bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f16(std::basic_ostringstream<T1, T2, T3> &a0, short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f17(std::basic_ostringstream<T1, T2, T3> &a0, unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f18(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f19(std::basic_ostringstream<T1, T2, T3> &a0, unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f20(std::basic_ostringstream<T1, T2, T3> &a0, long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f21(std::basic_ostringstream<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f22(std::basic_ostringstream<T1, T2, T3> &a0, double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f23(std::basic_ostringstream<T1, T2, T3> &a0, float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f24(std::basic_ostringstream<T1, T2, T3> &a0, long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f25(std::basic_ostringstream<T1, T2, T3> &a0, const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f26(std::basic_ostringstream<T1, T2, T3> &a0,
    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f27(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::char_type a1) {
  return a0.put(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f28(std::basic_ostringstream<T1, T2, T3> &a0,
    const typename std::basic_ostringstream<T1, T2, T3>::char_type *a1,
    std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f29(std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.flush();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::pos_type
f30(std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f31(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::pos_type a1) {
  return a0.seekp(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f32(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::off_type a1,
    typename std::basic_ostringstream<T1, T2, T3>::seekdir a2) {
  return a0.seekp(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f33(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f34(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::iostate
f35(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f36(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f37(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
bool f38(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f39(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f40(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f41(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::iostate
f42(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f43(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *
f44(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f45(std::basic_ostringstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::char_type
f46(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::char_type
f47(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f48(std::basic_ostringstream<T1, T2, T3> &a0,
                const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f49(const std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::char_type
f50(const std::basic_ostringstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f51(std::basic_ostringstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_ostringstream<T1, T2, T3>::event,
                    std::basic_ostringstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f52(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f53(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f54(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::fmtflags
f55(std::basic_ostringstream<T1, T2, T3> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_ostringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f56(std::basic_ostringstream<T1, T2, T3> &a0,
         typename std::basic_ostringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f57(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f58(std::basic_ostringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f59(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f60(std::basic_ostringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f61(bool a0) {
  return std::basic_ostringstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f62(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f63() {
  return std::basic_ostringstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f64(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f65(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}
