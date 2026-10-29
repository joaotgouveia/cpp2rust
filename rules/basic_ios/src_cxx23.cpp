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
typename std::basic_ios<T1, T2>::iostate f10(const std::basic_ios<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f11(std::basic_ios<T1, T2> &a0,
         typename std::basic_ios<T1, T2>::iostate a1) {
  return a0.exceptions(
      std::forward<typename std::basic_ios<T1, T2>::iostate>(a1));
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

template <typename T1, typename T2>
std::basic_ios<T1, T2> &f17(std::basic_ios<T1, T2> &a0,
                            const std::basic_ios<T1, T2> &a1) {
  return a0.copyfmt(a1);
}

template <typename T1, typename T2> T1 f18(const std::basic_ios<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2> T1 f19(std::basic_ios<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f20(std::basic_ios<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f21(const std::basic_ios<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f22(const std::basic_ios<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f23(std::basic_ios<T1, T2> &a0,
         void (*a1)(typename std::basic_ios<T1, T2>::event,
                    std::basic_ios<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
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
std::locale f34(const std::basic_ios<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f35() {
  return std::basic_ios<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f36(std::basic_ios<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f37(std::basic_ios<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}
