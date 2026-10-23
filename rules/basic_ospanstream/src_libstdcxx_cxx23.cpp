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
  return std::basic_ospanstream<T1, T2>(a0, a1);
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
  return a0.span(a1);
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
std::basic_ospanstream<T1, T2> &
f27(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::char_type a1) {
  return a0.put(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f28(std::basic_ospanstream<T1, T2> &a0,
    const typename std::basic_ospanstream<T1, T2>::char_type *a1,
    std::streamsize a2) {
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
  return a0.seekp(a1);
}

template <typename T1, typename T2>
std::basic_ospanstream<T1, T2> &
f32(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::off_type a1,
    typename std::basic_ospanstream<T1, T2>::seekdir a2) {
  return a0.seekp(a1, a2);
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
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f37(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
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
  return a0.exceptions(a1);
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
typename std::basic_ospanstream<T1, T2>::char_type
f46(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::char_type
f47(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f48(std::basic_ospanstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f49(const std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::char_type
f50(const std::basic_ospanstream<T1, T2> &a0, char a1) {
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
typename std::basic_ospanstream<T1, T2>::fmtflags
f52(const std::basic_ospanstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags
f53(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags
f54(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_ospanstream<T1, T2>::fmtflags
f55(std::basic_ospanstream<T1, T2> &a0,
    typename std::basic_ospanstream<T1, T2>::fmtflags a1,
    typename std::basic_ospanstream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f56(std::basic_ospanstream<T1, T2> &a0,
         typename std::basic_ospanstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
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
