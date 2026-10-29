#include <algorithm>
#include <ios>
#include <utility>

using t1 = std::ctype<char>;

using t2 = std::ctype<char>::char_type;

bool f1(const std::ctype<char> &a0, std::ctype_base::mask a1, char a2) {
  return a0.is(a1, a2);
}

const char *f2(const std::ctype<char> &a0, const char *a1, const char *a2,
               std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const char *f3(const std::ctype<char> &a0, std::ctype_base::mask a1,
               const char *a2, const char *a3) {
  return a0.scan_is(a1, a2, a3);
}

const char *f4(const std::ctype<char> &a0, std::ctype_base::mask a1,
               const char *a2, const char *a3) {
  return a0.scan_not(a1, a2, a3);
}

std::ctype<char>::char_type f5(const std::ctype<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.toupper(a1);
}

const std::ctype<char>::char_type *f6(const std::ctype<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.toupper(a1, a2);
}

std::ctype<char>::char_type f7(const std::ctype<char> &a0,
                               std::ctype<char>::char_type a1) {
  return a0.tolower(a1);
}

const std::ctype<char>::char_type *f8(const std::ctype<char> &a0,
                                      std::ctype<char>::char_type *a1,
                                      const std::ctype<char>::char_type *a2) {
  return a0.tolower(a1, a2);
}

std::ctype<char>::char_type f9(const std::ctype<char> &a0, char a1) {
  return a0.widen(a1);
}

const char *f10(const std::ctype<char> &a0, const char *a1, const char *a2,
                std::ctype<char>::char_type *a3) {
  return a0.widen(a1, a2, a3);
}

char f11(const std::ctype<char> &a0, std::ctype<char>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

const std::ctype<char>::char_type *f12(const std::ctype<char> &a0,
                                       const std::ctype<char>::char_type *a1,
                                       const std::ctype<char>::char_type *a2,
                                       char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

const std::ctype_base::mask *f13(const std::ctype<char> &a0) {
  return a0.table();
}

const std::ctype_base::mask *f14() { return std::ctype<char>::classic_table(); }

const std::locale::id &f15() { return std::ctype<char>::id; }

std::size_t f16() { return std::ctype<char>::table_size; }

using t3 = std::ctype<wchar_t>;

using t4 = std::ctype<wchar_t>::char_type;

bool f17(const std::ctype<wchar_t> &a0, std::ctype_base::mask a1, wchar_t a2) {
  return a0.is(a1, a2);
}

const wchar_t *f18(const std::ctype<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const wchar_t *f19(const std::ctype<wchar_t> &a0, std::ctype_base::mask a1,
                   const wchar_t *a2, const wchar_t *a3) {
  return a0.scan_is(a1, a2, a3);
}

const wchar_t *f20(const std::ctype<wchar_t> &a0, std::ctype_base::mask a1,
                   const wchar_t *a2, const wchar_t *a3) {
  return a0.scan_not(a1, a2, a3);
}

wchar_t f21(const std::ctype<wchar_t> &a0, wchar_t a1) {
  return a0.toupper(a1);
}

const wchar_t *f22(const std::ctype<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.toupper(a1, a2);
}

wchar_t f23(const std::ctype<wchar_t> &a0, wchar_t a1) {
  return a0.tolower(a1);
}

const wchar_t *f24(const std::ctype<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.tolower(a1, a2);
}

wchar_t f25(const std::ctype<wchar_t> &a0, char a1) { return a0.widen(a1); }

const char *f26(const std::ctype<wchar_t> &a0, const char *a1, const char *a2,
                wchar_t *a3) {
  return a0.widen(a1, a2, a3);
}

char f27(const std::ctype<wchar_t> &a0, wchar_t a1, char a2) {
  return a0.narrow(a1, a2);
}

const wchar_t *f28(const std::ctype<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

const std::locale::id &f29() { return std::ctype<wchar_t>::id; }

template <typename T1> using t5 = std::ctype<T1>;

template <typename T1> using t6 = typename std::ctype<T1>::char_type;

template <typename T1> using t7 = typename std::ctype<T1>::mask;

template <typename T1> std::ctype<T1> f30(std::size_t a0) {
  return std::ctype<T1>(a0);
}

template <typename T1>
bool f31(const std::ctype<T1> &a0, unsigned short a1, T1 a2) {
  return a0.is(a1, std::move(a2));
}

template <typename T1>
T1 const *f32(const std::ctype<T1> &a0, T1 const *a1, T1 const *a2,
              unsigned short *a3) {
  return a0.is(a1, a2, a3);
}

template <typename T1>
T1 const *f33(const std::ctype<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_is(a1, a2, a3);
}

template <typename T1>
T1 const *f34(const std::ctype<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_not(a1, a2, a3);
}

template <typename T1> T1 f35(const std::ctype<T1> &a0, T1 a1) {
  return a0.toupper(std::move(a1));
}

template <typename T1>
T1 const *f36(const std::ctype<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.toupper(a1, a2);
}

template <typename T1> T1 f37(const std::ctype<T1> &a0, T1 a1) {
  return a0.tolower(std::move(a1));
}

template <typename T1>
T1 const *f38(const std::ctype<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.tolower(a1, a2);
}

template <typename T1> T1 f39(const std::ctype<T1> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1>
const char *f40(const std::ctype<T1> &a0, const char *a1, const char *a2,
                T1 *a3) {
  return a0.widen(a1, a2, a3);
}

template <typename T1> char f41(const std::ctype<T1> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1>
T1 const *f42(const std::ctype<T1> &a0, T1 const *a1, T1 const *a2, char a3,
              char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

template <typename T1> const std::locale::id &f43() {
  return std::ctype<T1>::id;
}
