#include <fstream>
#include <ios>
#include <ostream>
#include <string>
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
std::basic_ifstream<T1, T2> f2(std::basic_ifstream<T1, T2> &&a0) {
  return std::basic_ifstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_ifstream<T1, T2> &f3(std::basic_ifstream<T1, T2> &a0,
                                std::basic_ifstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f4(std::basic_ifstream<T1, T2> &a0, std::basic_ifstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_filebuf<T1, T2> *f5(const std::basic_ifstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
bool f6(const std::basic_ifstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2> void f7(std::basic_ifstream<T1, T2> &a0) {
  return a0.close();
}

template <typename T1, typename T2>
std::streamsize f8(const std::basic_ifstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::int_type
f9(std::basic_ifstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::int_type
f10(std::basic_ifstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::streamsize f11(std::basic_ifstream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2> int f12(std::basic_ifstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_ifstream<T1, T2>::pos_type
f13(std::basic_ifstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
bool f14(const std::basic_ifstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f15(const std::basic_ifstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
bool f16(const std::basic_ifstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f17(const std::basic_ifstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f18(const std::basic_ifstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f19(const std::basic_ifstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f20(const std::basic_ifstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f21(std::basic_ifstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f22(const std::basic_ifstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f23(std::basic_ifstream<T1, T2> &a0, T1 a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f24(std::basic_ifstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f25(const std::basic_ifstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
T1 f26(const std::basic_ifstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f27(std::basic_ifstream<T1, T2> &a0,
         void (*a1)(typename std::basic_ifstream<T1, T2>::event,
                    std::basic_ifstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f28(const std::basic_ifstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f29(std::basic_ifstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f30(const std::basic_ifstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f31(std::basic_ifstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f32(bool a0) {
  return std::basic_ifstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f33(const std::basic_ifstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f34() {
  return std::basic_ifstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f35(std::basic_ifstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f36(std::basic_ifstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}
