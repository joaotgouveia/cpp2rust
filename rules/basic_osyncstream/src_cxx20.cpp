#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <ios>
#include <iosfwd>
#include <ostream>
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
   const typename std::basic_osyncstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_osyncstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3>
f2(typename std::basic_osyncstream<T1, T2, T3>::streambuf_type *a0) {
  return std::basic_osyncstream<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3>
f3(std::basic_ostream<typename std::basic_osyncstream<T1, T2, T3>::char_type,
                      typename std::basic_osyncstream<T1, T2, T3>::traits_type>
       &a0,
   const typename std::basic_osyncstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_osyncstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_osyncstream<T1, T2, T3>
f4(std::basic_ostream<typename std::basic_osyncstream<T1, T2, T3>::char_type,
                      typename std::basic_osyncstream<T1, T2, T3>::traits_type>
       &a0) {
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
typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type *
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
typename std::basic_osyncstream<T1, T2, T3>::pos_type
f10(std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2, typename T3>
bool f11(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f12(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::iostate
f13(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f14(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f15(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
bool f16(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f17(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f18(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f19(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::iostate
f20(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f21(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::char_type
f22(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::char_type
f23(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f24(std::basic_osyncstream<T1, T2, T3> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f25(const std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::char_type
f26(const std::basic_osyncstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f27(std::basic_osyncstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_osyncstream<T1, T2, T3>::event,
                    std::basic_osyncstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::fmtflags
f28(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::fmtflags
f29(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::fmtflags
f30(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_osyncstream<T1, T2, T3>::fmtflags
f31(std::basic_osyncstream<T1, T2, T3> &a0,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_osyncstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f32(std::basic_osyncstream<T1, T2, T3> &a0,
         typename std::basic_osyncstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f33(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f34(std::basic_osyncstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f35(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f36(std::basic_osyncstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f37(bool a0) {
  return std::basic_osyncstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f38(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f39() {
  return std::basic_osyncstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f40(std::basic_osyncstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f41(std::basic_osyncstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2, typename T3>
using t10 =
    typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::char_type;

template <typename T1, typename T2, typename T3>
using t11 = typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::int_type;

template <typename T1, typename T2, typename T3>
using t12 = typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::pos_type;

template <typename T1, typename T2, typename T3>
using t13 = typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::off_type;

template <typename T1, typename T2, typename T3>
using t14 =
    typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::traits_type;

template <typename T1, typename T2, typename T3>
using t15 =
    typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::allocator_type;

template <typename T1, typename T2, typename T3>
using t16 =
    typename std::basic_osyncstream<T1, T2, T3>::syncbuf_type::streambuf_type;

template <typename T1, typename T2, typename T3>
using t17 =
    typename std::basic_osyncstream<T1, T2, T3>::streambuf_type::char_type;

template <typename T1, typename T2, typename T3>
using t18 =
    typename std::basic_osyncstream<T1, T2, T3>::streambuf_type::traits_type;

template <typename T1, typename T2, typename T3>
using t19 =
    typename std::basic_osyncstream<T1, T2, T3>::streambuf_type::int_type;

template <typename T1, typename T2, typename T3>
using t20 =
    typename std::basic_osyncstream<T1, T2, T3>::streambuf_type::pos_type;

template <typename T1, typename T2, typename T3>
using t21 =
    typename std::basic_osyncstream<T1, T2, T3>::streambuf_type::off_type;
