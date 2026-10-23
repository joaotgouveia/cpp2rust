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
std::streamsize f4(std::basic_iostream<T1, T2> &a0, T1 *a1,
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
bool f9(const std::basic_iostream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f10(const std::basic_iostream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f11(const std::basic_iostream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f12(const std::basic_iostream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f13(const std::basic_iostream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f14(std::basic_iostream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f15(const std::basic_iostream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f16(std::basic_iostream<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2>
T1 f17(const std::basic_iostream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f18(std::basic_iostream<T1, T2> &a0, T1 a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f19(std::basic_iostream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f20(const std::basic_iostream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
T1 f21(const std::basic_iostream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f22(std::basic_iostream<T1, T2> &a0,
         void (*a1)(typename std::basic_iostream<T1, T2>::event,
                    std::basic_iostream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f23(const std::basic_iostream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f24(std::basic_iostream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f25(const std::basic_iostream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f26(std::basic_iostream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f27(bool a0) {
  return std::basic_iostream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f28(const std::basic_iostream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f29() {
  return std::basic_iostream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f30(std::basic_iostream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f31(std::basic_iostream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_iostream<T1, T2>::pos_type
f32(std::basic_iostream<T1, T2> &a0) {
  return a0.tellp();
}
