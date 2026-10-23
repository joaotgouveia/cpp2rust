#include <fstream>
#include <ios>
#include <ostream>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_fstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_fstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_fstream<T1, T2>::traits_type;

template <typename T1, typename T2>
using t4 = typename std::basic_fstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t5 = typename std::basic_fstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t6 = typename std::basic_fstream<T1, T2>::off_type;

template <typename T1, typename T2> std::basic_fstream<T1, T2> f1() {
  return std::basic_fstream<T1, T2>();
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2>
f2(const char *a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2>
f3(const std::string &a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> f4(std::basic_fstream<T1, T2> &&a0) {
  return std::basic_fstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f5(std::basic_fstream<T1, T2> &a0,
                               std::basic_fstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f6(std::basic_fstream<T1, T2> &a0, std::basic_fstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_filebuf<typename std::basic_fstream<T1, T2>::char_type,
                   typename std::basic_fstream<T1, T2>::traits_type> *
f7(const std::basic_fstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
bool f8(const std::basic_fstream<T1, T2> &a0) {
  return a0.is_open();
}

template <typename T1, typename T2>
void f9(std::basic_fstream<T1, T2> &a0, const char *a1,
        typename std::basic_fstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2>
void f10(std::basic_fstream<T1, T2> &a0, const std::string &a1,
         typename std::basic_fstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}

template <typename T1, typename T2> void f11(std::basic_fstream<T1, T2> &a0) {
  return a0.close();
}

template <typename T1, typename T2>
std::streamsize f12(const std::basic_fstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::int_type
f13(std::basic_fstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::int_type
f14(std::basic_fstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::streamsize f15(std::basic_fstream<T1, T2> &a0,
                    typename std::basic_fstream<T1, T2>::char_type *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2> int f16(std::basic_fstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::pos_type
f17(std::basic_fstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
bool f18(const std::basic_fstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f19(const std::basic_fstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::iostate
f20(const std::basic_fstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f21(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.clear(a1);
}

template <typename T1, typename T2>
void f22(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.setstate(a1);
}

template <typename T1, typename T2>
bool f23(const std::basic_fstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f24(const std::basic_fstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f25(const std::basic_fstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f26(const std::basic_fstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::iostate
f27(const std::basic_fstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f28(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::iostate a1) {
  return a0.exceptions(a1);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::char_type
f29(const std::basic_fstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::char_type
f30(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::char_type a1) {
  return a0.fill(a1);
}

template <typename T1, typename T2>
std::locale f31(std::basic_fstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f32(const std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::char_type
f33(const std::basic_fstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f34(std::basic_fstream<T1, T2> &a0,
         void (*a1)(typename std::basic_fstream<T1, T2>::event,
                    std::basic_fstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::fmtflags
f35(const std::basic_fstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::fmtflags
f36(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.flags(a1);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::fmtflags
f37(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.setf(a1);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::fmtflags
f38(std::basic_fstream<T1, T2> &a0,
    typename std::basic_fstream<T1, T2>::fmtflags a1,
    typename std::basic_fstream<T1, T2>::fmtflags a2) {
  return a0.setf(a1, a2);
}

template <typename T1, typename T2>
void f39(std::basic_fstream<T1, T2> &a0,
         typename std::basic_fstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(a1);
}

template <typename T1, typename T2>
std::streamsize f40(const std::basic_fstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f41(std::basic_fstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f42(const std::basic_fstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f43(std::basic_fstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f44(bool a0) {
  return std::basic_fstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f45(const std::basic_fstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f46() {
  return std::basic_fstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f47(std::basic_fstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f48(std::basic_fstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}

template <typename T1, typename T2>
typename std::basic_fstream<T1, T2>::pos_type
f49(std::basic_fstream<T1, T2> &a0) {
  return a0.tellp();
}
