#include "../common/init.h"
#include "../common/traits.h"
#include <iosfwd>
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
f3(std::basic_istringstream<T1, T2, T3> &&a0) {
  return std::basic_istringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3> &
f4(std::basic_istringstream<T1, T2, T3> &a0,
   std::basic_istringstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f5(std::basic_istringstream<T1, T2, T3> &a0,
        std::basic_istringstream<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f6(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::int_type
f7(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.get();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::int_type
f8(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.peek();
}

template <typename T1, typename T2, typename T3>
std::streamsize f9(std::basic_istringstream<T1, T2, T3> &a0,
                   typename std::basic_istringstream<T1, T2, T3>::char_type *a1,
                   std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f10(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.sync();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::pos_type
f11(std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2, typename T3>
bool f12(const std::basic_istringstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f13(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f14(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2, typename T3>
void f15(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2, typename T3>
void f16(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2, typename T3>
bool f17(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f18(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f19(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f20(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::iostate
f21(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2, typename T3>
void f22(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f23(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f24(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2, typename T3>
std::locale f25(std::basic_istringstream<T1, T2, T3> &a0,
                const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f26(const std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::char_type
f27(const std::basic_istringstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f28(std::basic_istringstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_istringstream<T1, T2, T3>::event,
                    std::basic_istringstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f29(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.flags();
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f30(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f31(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_istringstream<T1, T2, T3>::fmtflags
f32(std::basic_istringstream<T1, T2, T3> &a0,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a1,
    typename std::basic_istringstream<T1, T2, T3>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f33(std::basic_istringstream<T1, T2, T3> &a0,
         typename std::basic_istringstream<T1, T2, T3>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f34(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f35(std::basic_istringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f36(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f37(std::basic_istringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f38(bool a0) {
  return std::basic_istringstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f39(const std::basic_istringstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f40() {
  return std::basic_istringstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f41(std::basic_istringstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f42(std::basic_istringstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}
