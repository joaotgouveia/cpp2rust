#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

using t1 = std::ctype_byname<char>;

bool f1(const std::ctype_byname<char> &a0, std::ctype_base::mask a1, char a2) {
  return a0.is(a1, a2);
}

const char *f2(const std::ctype_byname<char> &a0, const char *a1,
               const char *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const char *f3(const std::ctype_byname<char> &a0, std::ctype_base::mask a1,
               const char *a2, const char *a3) {
  return a0.scan_is(a1, a2, a3);
}

const char *f4(const std::ctype_byname<char> &a0, std::ctype_base::mask a1,
               const char *a2, const char *a3) {
  return a0.scan_not(a1, a2, a3);
}

std::ctype<char>::char_type f5(const std::ctype_byname<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.toupper(a1);
}

const std::ctype<char>::char_type *f6(const std::ctype_byname<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.toupper(a1, a2);
}

std::ctype<char>::char_type f7(const std::ctype_byname<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.tolower(a1);
}

const std::ctype<char>::char_type *f8(const std::ctype_byname<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.tolower(a1, a2);
}

std::ctype<char>::char_type f9(const std::ctype_byname<char> &a0, char a1) {
  return a0.widen(a1);
}

const char *f10(const std::ctype_byname<char> &a0, const char *a1,
                const char *a2, std::ctype<char>::char_type *a3) {
  return a0.widen(a1, a2, a3);
}

char f11(const std::ctype_byname<char> &a0, std::ctype<char>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

const std::ctype<char>::char_type *f12(const std::ctype_byname<char> &a0,
                                       const std::ctype<char>::char_type *a1,
                                       const std::ctype<char>::char_type *a2,
                                       char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

const std::ctype_base::mask *f13(const std::ctype_byname<char> &a0) {
  return a0.table();
}

const std::ctype_base::mask *f14() {
  return std::ctype_byname<char>::classic_table();
}

const std::locale::id &f15() { return std::ctype_byname<char>::id; }

std::size_t f16() { return std::ctype_byname<char>::table_size; }

std::ctype_base::mask f17() { return std::ctype_byname<char>::upper; }

std::ctype_base::mask f18() { return std::ctype_byname<char>::lower; }

std::ctype_base::mask f19() { return std::ctype_byname<char>::alpha; }

std::ctype_base::mask f20() { return std::ctype_byname<char>::digit; }

std::ctype_base::mask f21() { return std::ctype_byname<char>::xdigit; }

std::ctype_base::mask f22() { return std::ctype_byname<char>::space; }

std::ctype_base::mask f23() { return std::ctype_byname<char>::print; }

std::ctype_base::mask f24() { return std::ctype_byname<char>::graph; }

std::ctype_base::mask f25() { return std::ctype_byname<char>::cntrl; }

std::ctype_base::mask f26() { return std::ctype_byname<char>::punct; }

std::ctype_base::mask f27() { return std::ctype_byname<char>::alnum; }

std::ctype_base::mask f28() { return std::ctype_byname<char>::blank; }

using t2 = std::ctype_byname<wchar_t>;

const std::locale::id &f29() { return std::ctype_byname<wchar_t>::id; }

bool f30(const std::ctype_byname<wchar_t> &a0, std::ctype_base::mask a1,
         wchar_t a2) {
  return a0.is(a1, a2);
}

const wchar_t *f31(const std::ctype_byname<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const wchar_t *f32(const std::ctype_byname<wchar_t> &a0,
                   std::ctype_base::mask a1, const wchar_t *a2,
                   const wchar_t *a3) {
  return a0.scan_is(a1, a2, a3);
}

const wchar_t *f33(const std::ctype_byname<wchar_t> &a0,
                   std::ctype_base::mask a1, const wchar_t *a2,
                   const wchar_t *a3) {
  return a0.scan_not(a1, a2, a3);
}

wchar_t f34(const std::ctype_byname<wchar_t> &a0, wchar_t a1) {
  return a0.toupper(a1);
}

const wchar_t *f35(const std::ctype_byname<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.toupper(a1, a2);
}

wchar_t f36(const std::ctype_byname<wchar_t> &a0, wchar_t a1) {
  return a0.tolower(a1);
}

const wchar_t *f37(const std::ctype_byname<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.tolower(a1, a2);
}

wchar_t f38(const std::ctype_byname<wchar_t> &a0, char a1) {
  return a0.widen(a1);
}

const char *f39(const std::ctype_byname<wchar_t> &a0, const char *a1,
                const char *a2, wchar_t *a3) {
  return a0.widen(a1, a2, a3);
}

char f40(const std::ctype_byname<wchar_t> &a0, wchar_t a1, char a2) {
  return a0.narrow(a1, a2);
}

const wchar_t *f41(const std::ctype_byname<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

std::ctype_base::mask f42() { return std::ctype_byname<wchar_t>::upper; }

std::ctype_base::mask f43() { return std::ctype_byname<wchar_t>::lower; }

std::ctype_base::mask f44() { return std::ctype_byname<wchar_t>::alpha; }

std::ctype_base::mask f45() { return std::ctype_byname<wchar_t>::digit; }

std::ctype_base::mask f46() { return std::ctype_byname<wchar_t>::xdigit; }

std::ctype_base::mask f47() { return std::ctype_byname<wchar_t>::space; }

std::ctype_base::mask f48() { return std::ctype_byname<wchar_t>::print; }

std::ctype_base::mask f49() { return std::ctype_byname<wchar_t>::graph; }

std::ctype_base::mask f50() { return std::ctype_byname<wchar_t>::cntrl; }

std::ctype_base::mask f51() { return std::ctype_byname<wchar_t>::punct; }

std::ctype_base::mask f52() { return std::ctype_byname<wchar_t>::alnum; }

std::ctype_base::mask f53() { return std::ctype_byname<wchar_t>::blank; }

template <typename T1> using t3 = std::ctype_byname<T1>;

template <typename T1> using t4 = typename std::ctype_byname<T1>::mask;

template <typename T1>
std::ctype_byname<T1> f54(const char *a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1>
std::ctype_byname<T1> f55(const std::string &a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1> const std::locale::id &f56() {
  return std::ctype_byname<T1>::id;
}

template <typename T1>
bool f57(const std::ctype_byname<T1> &a0, unsigned short a1, T1 a2) {
  return a0.is(a1, std::move(a2));
}

template <typename T1>
T1 const *f58(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              unsigned short *a3) {
  return a0.is(a1, a2, a3);
}

template <typename T1>
T1 const *f59(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_is(a1, a2, a3);
}

template <typename T1>
T1 const *f60(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_not(a1, a2, a3);
}

template <typename T1> T1 f61(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.toupper(std::move(a1));
}

template <typename T1>
T1 const *f62(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.toupper(a1, a2);
}

template <typename T1> T1 f63(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.tolower(std::move(a1));
}

template <typename T1>
T1 const *f64(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.tolower(a1, a2);
}

template <typename T1> T1 f65(const std::ctype_byname<T1> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1>
const char *f66(const std::ctype_byname<T1> &a0, const char *a1, const char *a2,
                T1 *a3) {
  return a0.widen(a1, a2, a3);
}

template <typename T1>
char f67(const std::ctype_byname<T1> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1>
T1 const *f68(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

template <typename T1> unsigned short f69() {
  return std::ctype_byname<T1>::upper;
}

template <typename T1> unsigned short f70() {
  return std::ctype_byname<T1>::lower;
}

template <typename T1> unsigned short f71() {
  return std::ctype_byname<T1>::alpha;
}

template <typename T1> unsigned short f72() {
  return std::ctype_byname<T1>::digit;
}

template <typename T1> unsigned short f73() {
  return std::ctype_byname<T1>::xdigit;
}

template <typename T1> unsigned short f74() {
  return std::ctype_byname<T1>::space;
}

template <typename T1> unsigned short f75() {
  return std::ctype_byname<T1>::print;
}

template <typename T1> unsigned short f76() {
  return std::ctype_byname<T1>::graph;
}

template <typename T1> unsigned short f77() {
  return std::ctype_byname<T1>::cntrl;
}

template <typename T1> unsigned short f78() {
  return std::ctype_byname<T1>::punct;
}

template <typename T1> unsigned short f79() {
  return std::ctype_byname<T1>::alnum;
}

template <typename T1> unsigned short f80() {
  return std::ctype_byname<T1>::blank;
}
