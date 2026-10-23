#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_iostream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_iostream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_iostream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_iostream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_iostream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_iostream<T1, T2>::traits_type;

template <typename T1, typename T2>
std::streamsize f1(const std::basic_iostream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::int_type
f2(std::basic_iostream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::int_type
f3(std::basic_iostream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::streamsize f4(std::basic_iostream<T1, T2> &a0,
                   typename std::basic_iostream<T1, T2>::char_type *a1,
                   std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2> int f5(std::basic_iostream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::pos_type
f6(std::basic_iostream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
bool f7(const std::basic_iostream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f8(const std::basic_iostream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::iostate
f9(const std::basic_iostream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f10(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f11(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
bool f12(const std::basic_iostream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f13(const std::basic_iostream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f14(const std::basic_iostream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f15(const std::basic_iostream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::iostate
f16(const std::basic_iostream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f17(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::char_type
f18(const std::basic_iostream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::char_type
f19(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f20(std::basic_iostream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f21(const std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::char_type
f22(const std::basic_iostream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f23(std::basic_iostream<T1, T2> &a0,
         void (*a1)(typename std::basic_iostream<T1, T2>::event,
                    std::basic_iostream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f24(const std::basic_iostream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f25(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f26(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::fmtflags
f27(std::basic_iostream<T1, T2> &a0,
    typename std::basic_iostream<T1, T2>::fmtflags a1,
    typename std::basic_iostream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f28(std::basic_iostream<T1, T2> &a0,
         typename std::basic_iostream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::streamsize f29(const std::basic_iostream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f30(std::basic_iostream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f31(const std::basic_iostream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f32(std::basic_iostream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f33(bool a0) {
  return std::basic_iostream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f34(const std::basic_iostream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f35() {
  return std::basic_iostream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f36(std::basic_iostream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f37(std::basic_iostream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::pos_type
f38(std::basic_iostream<T1, T2> &a0) {
  return a0.tellp();
}
