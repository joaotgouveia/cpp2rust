#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ios<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ios<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ios<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ios<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ios<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ios<T1, T2>::traits_type;

template <typename T1, typename T2> bool f1(const std::basic_ios<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> bool f2(const std::basic_ios<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f3(const std::basic_ios<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f4(std::basic_ios<T1, T2> &a0,
        typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.clear(std::forward<typename std::basic_ios<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2>
void f5(std::basic_ios<T1, T2> &a0,
        typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.setstate(
      std::forward<typename std::basic_ios<T1, T2>::iostate>(a1));
}

template <typename T1, typename T2> bool f6(const std::basic_ios<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2> bool f7(const std::basic_ios<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2> bool f8(const std::basic_ios<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2> bool f9(const std::basic_ios<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
std::basic_ios<T1, T2> f12(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_ios<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f13(const std::basic_ios<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f14(std::basic_ios<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f15(const std::basic_ios<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f16(std::basic_ios<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}

template <typename T1, typename T2> T1 f18(const std::basic_ios<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2> T1 f19(std::basic_ios<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f24(const std::basic_ios<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags f25(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.flags(std::forward<typename std::basic_ios<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f26(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.setf(std::forward<typename std::basic_ios<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags f27(std::basic_ios<T1, T2> &a0,
                            typename std::basic_ios<T1, T2>::fmtflags a1,
                            typename std::basic_ios<T1, T2>::fmtflags a2) {
  return a0.setf(std::forward<typename std::basic_ios<T1, T2>::fmtflags>(a1),
                 std::forward<typename std::basic_ios<T1, T2>::fmtflags>(a2));
}

template <typename T1, typename T2>
void f28(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::fmtflags a1) {
  return a0.unsetf(std::forward<typename std::basic_ios<T1, T2>::fmtflags>(a1));
}

template <typename T1, typename T2>
std::streamsize f29(const std::basic_ios<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f30(std::basic_ios<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f31(const std::basic_ios<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f32(std::basic_ios<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f33(bool a0) {
  return std::basic_ios<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f38() {
  return std::basic_ios<T1, T2>::boolalpha;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f39() {
  return std::basic_ios<T1, T2>::dec;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f40() {
  return std::basic_ios<T1, T2>::fixed;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f41() {
  return std::basic_ios<T1, T2>::hex;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f42() {
  return std::basic_ios<T1, T2>::internal;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f43() {
  return std::basic_ios<T1, T2>::left;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f44() {
  return std::basic_ios<T1, T2>::oct;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f45() {
  return std::basic_ios<T1, T2>::right;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f46() {
  return std::basic_ios<T1, T2>::scientific;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f47() {
  return std::basic_ios<T1, T2>::showbase;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f48() {
  return std::basic_ios<T1, T2>::showpoint;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f49() {
  return std::basic_ios<T1, T2>::showpos;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f50() {
  return std::basic_ios<T1, T2>::skipws;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f51() {
  return std::basic_ios<T1, T2>::unitbuf;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f52() {
  return std::basic_ios<T1, T2>::uppercase;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f53() {
  return std::basic_ios<T1, T2>::adjustfield;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f54() {
  return std::basic_ios<T1, T2>::basefield;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::fmtflags f55() {
  return std::basic_ios<T1, T2>::floatfield;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f56() {
  return std::basic_ios<T1, T2>::badbit;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f57() {
  return std::basic_ios<T1, T2>::eofbit;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f58() {
  return std::basic_ios<T1, T2>::failbit;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::iostate f59() {
  return std::basic_ios<T1, T2>::goodbit;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f60() {
  return std::basic_ios<T1, T2>::app;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f61() {
  return std::basic_ios<T1, T2>::ate;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f62() {
  return std::basic_ios<T1, T2>::binary;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f63() {
  return std::basic_ios<T1, T2>::in;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f64() {
  return std::basic_ios<T1, T2>::out;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f65() {
  return std::basic_ios<T1, T2>::trunc;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::openmode f66() {
  return std::basic_ios<T1, T2>::noreplace;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::seekdir f67() {
  return std::basic_ios<T1, T2>::beg;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::seekdir f68() {
  return std::basic_ios<T1, T2>::cur;
}

template <typename T1, typename T2>
typename std::basic_ios<T1, T2>::seekdir f69() {
  return std::basic_ios<T1, T2>::end;
}
