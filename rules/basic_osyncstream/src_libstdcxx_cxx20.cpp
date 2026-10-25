#include <algorithm>
#include <barrier>
#include <syncstream>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_osyncstream<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_osyncstream<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_osyncstream<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_osyncstream<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_osyncstream<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_osyncstream<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_osyncstream<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
using t8 = typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type;

template <typename T1, typename T2, typename T3>
using t9 = typename std::basic_osyncstream<T1, T2, T3>::streambuf_type;

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3>
f1(typename std::basic_osyncstream<T1, T2, T3>::streambuf_type *a0,
   const T3 &a1) {
  return std::basic_osyncstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3>
f2(typename std::basic_osyncstream<T1, T2, T3>::streambuf_type *a0) {
  return std::basic_osyncstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> f3(std::basic_ostream<T1, T2> &a0,
                                      const T3 &a1) {
  return std::basic_osyncstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> f4(std::basic_ostream<T1, T2> &a0) {
  return std::basic_osyncstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> f5(std::basic_osyncstream<T1, T2, T3> &&a0) {
  return std::basic_osyncstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f6(std::basic_osyncstream<T1, T2, T3> &a0,
   std::basic_osyncstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_syncbuf<T1, T2, T3> *
f7(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::streambuf_type *
f8(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.get_wrapped();
}

template <typename T1, typename T2, typename T3>
void f9(std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.emit();
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f10(std::basic_osyncstream<T1, T2, T3> &a0,
    std::basic_osyncstream<T1, T2, T3> &(*a1)(
        std::basic_osyncstream<T1, T2, T3> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f11(std::basic_osyncstream<T1, T2, T3> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f12(std::basic_osyncstream<T1, T2, T3> &a0,
                                        std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f13(std::basic_osyncstream<T1, T2, T3> &a0,
                                        long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f14(std::basic_osyncstream<T1, T2, T3> &a0,
                                        unsigned long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f15(std::basic_osyncstream<T1, T2, T3> &a0,
                                        bool a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f16(std::basic_osyncstream<T1, T2, T3> &a0,
                                        short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f17(std::basic_osyncstream<T1, T2, T3> &a0,
                                        unsigned short a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f18(std::basic_osyncstream<T1, T2, T3> &a0,
                                        int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f19(std::basic_osyncstream<T1, T2, T3> &a0,
                                        unsigned int a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f20(std::basic_osyncstream<T1, T2, T3> &a0,
                                        long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f21(std::basic_osyncstream<T1, T2, T3> &a0,
                                        unsigned long long a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f22(std::basic_osyncstream<T1, T2, T3> &a0,
                                        double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f23(std::basic_osyncstream<T1, T2, T3> &a0,
                                        float a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f24(std::basic_osyncstream<T1, T2, T3> &a0,
                                        long double a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f25(std::basic_osyncstream<T1, T2, T3> &a0,
                                        const void *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f26(std::basic_osyncstream<T1, T2, T3> &a0,
                                        std::nullptr_t a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f27(std::basic_osyncstream<T1, T2, T3> &a0,
                                        std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f28(std::basic_osyncstream<T1, T2, T3> &a0,
                                        T1 a1) {
  return a0.put(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &f29(std::basic_osyncstream<T1, T2, T3> &a0,
                                        T1 const *a1, std::streamsize a2) {
  return a0.write(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f30(std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.flush();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::pos_type
f31(std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f32(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::pos_type a1) {
  return a0.seekp(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::pos_type>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3> &
f33(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::off_type a1,
    typename std::basic_osyncstream<T1, T2, T3>::seekdir a2) {
  return a0.seekp(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::off_type>(a1),
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::seekdir>(a2));
}

template <typename T1, typename T2, typename T3>
bool f34(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f35(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::iostate
f36(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f37(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.clear(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
void f38(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
bool f39(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f40(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f41(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f42(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::iostate
f43(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f44(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::iostate>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f45(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f46(std::basic_osyncstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2, typename T3>
T1 f47(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
T1 f48(std::basic_osyncstream<T1, T2, T3> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::locale f49(std::basic_osyncstream<T1, T2, T3> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f50(const std::basic_osyncstream<T1, T2, T3> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f51(const std::basic_osyncstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f52(std::basic_osyncstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_osyncstream<T1, T2, T3>::event,
                    std::basic_osyncstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags f53(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f54(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f55(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::ios_base::fmtflags
f56(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::fmtflags>(a1),
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::fmtflags>(a2));
}

template <typename T1, typename T2, typename T3>
void f57(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(
      std::forward<typename std::basic_osyncstream<T1, T2, T3>::fmtflags>(a1));
}

template <typename T1, typename T2, typename T3>
std::streamsize f58(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f59(std::basic_osyncstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f60(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f61(std::basic_osyncstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f62(bool a0) {
  return std::basic_osyncstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f63(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f64() {
  return std::basic_osyncstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f65(std::basic_osyncstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f66(std::basic_osyncstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}
