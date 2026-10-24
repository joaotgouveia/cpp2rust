#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::basic_ostringstream<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::basic_ostringstream<T1, T2, T3>::char_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::basic_ostringstream<T1, T2, T3>::traits_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::basic_ostringstream<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::basic_ostringstream<T1, T2, T3>::int_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::basic_ostringstream<T1, T2, T3>::pos_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::basic_ostringstream<T1, T2, T3>::off_type;

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> f1() {
  return std::basic_ostringstream<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f2(std::basic_ostringstream<T1, T2, T3> &&a0) {
  return std::basic_ostringstream<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f3(std::basic_ostringstream<T1, T2, T3> &a0,
   std::basic_ostringstream<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f4(std::basic_ostringstream<T1, T2, T3> &a0,
        std::basic_ostringstream<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3> *
f5(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f6(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f7(std::basic_ostringstream<T1, T2, T3> &a0,
        const std::basic_string<T1, T2, T3> &a1) {
  return a0.str(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_ostringstream<T1, T2, T3>::pos_type
f8(std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.tellp();
}

template <typename T1, typename T2, typename T3>
bool f9(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f10(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2, typename T3>
bool f11(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.good();
}

template <typename T1, typename T2, typename T3>
bool f12(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.eof();
}

template <typename T1, typename T2, typename T3>
bool f13(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.fail();
}

template <typename T1, typename T2, typename T3>
bool f14(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.bad();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *
f15(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> *f16(std::basic_ostringstream<T1, T2, T3> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2, typename T3>
T1 f17(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.fill();
}

template <typename T1, typename T2, typename T3>
T1 f18(std::basic_ostringstream<T1, T2, T3> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::locale f19(std::basic_ostringstream<T1, T2, T3> &a0,
                const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2, typename T3>
char f20(const std::basic_ostringstream<T1, T2, T3> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f21(const std::basic_ostringstream<T1, T2, T3> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2, typename T3>
void f22(std::basic_ostringstream<T1, T2, T3> &a0,
         void (*a1)(typename std::basic_ostringstream<T1, T2, T3>::event,
                    std::basic_ostringstream<T1, T2, T3> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::streamsize f23(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.precision();
}

template <typename T1, typename T2, typename T3>
std::streamsize f24(std::basic_ostringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2, typename T3>
std::streamsize f25(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.width();
}

template <typename T1, typename T2, typename T3>
std::streamsize f26(std::basic_ostringstream<T1, T2, T3> &a0,
                    std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2, typename T3> bool f27(bool a0) {
  return std::basic_ostringstream<T1, T2, T3>::sync_with_stdio(a0);
}

template <typename T1, typename T2, typename T3>
std::locale f28(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2, typename T3> int f29() {
  return std::basic_ostringstream<T1, T2, T3>::xalloc();
}

template <typename T1, typename T2, typename T3>
long &f30(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2, typename T3>
void *&f31(std::basic_ostringstream<T1, T2, T3> &a0, int a1) {
  return a0.pword(a1);
}
