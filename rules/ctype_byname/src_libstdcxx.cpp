#include <algorithm>
#include <ios>
#include <string>
#include <utility>

bool f10(const std::ctype_byname<char> &a0, std::ctype_base::mask a1, char a2) {
  return a0.is(a1, a2);
}

const char *f11(const std::ctype_byname<char> &a0, const char *a1,
                const char *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const char *f12(const std::ctype_byname<char> &a0, std::ctype_base::mask a1,
                const char *a2, const char *a3) {
  return a0.scan_is(a1, a2, a3);
}

const char *f13(const std::ctype_byname<char> &a0, std::ctype_base::mask a1,
                const char *a2, const char *a3) {
  return a0.scan_not(a1, a2, a3);
}

const std::ctype<char>::char_type *f14(const std::ctype_byname<char> &a0,
                                       const std::ctype<char>::char_type *a1,
                                       const std::ctype<char>::char_type *a2,
                                       char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

bool f15(const std::ctype_byname<wchar_t> &a0, std::ctype_base::mask a1,
         wchar_t a2) {
  return a0.is(a1, a2);
}

const wchar_t *f16(const std::ctype_byname<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const wchar_t *f17(const std::ctype_byname<wchar_t> &a0,
                   std::ctype_base::mask a1, const wchar_t *a2,
                   const wchar_t *a3) {
  return a0.scan_is(a1, a2, a3);
}

const wchar_t *f18(const std::ctype_byname<wchar_t> &a0,
                   std::ctype_base::mask a1, const wchar_t *a2,
                   const wchar_t *a3) {
  return a0.scan_not(a1, a2, a3);
}

wchar_t f19(const std::ctype_byname<wchar_t> &a0, wchar_t a1) {
  return a0.toupper(a1);
}

const wchar_t *f20(const std::ctype_byname<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.toupper(a1, a2);
}

wchar_t f21(const std::ctype_byname<wchar_t> &a0, wchar_t a1) {
  return a0.tolower(a1);
}

const wchar_t *f22(const std::ctype_byname<wchar_t> &a0, wchar_t *a1,
                   const wchar_t *a2) {
  return a0.tolower(a1, a2);
}

wchar_t f23(const std::ctype_byname<wchar_t> &a0, char a1) {
  return a0.widen(a1);
}

const char *f24(const std::ctype_byname<wchar_t> &a0, const char *a1,
                const char *a2, wchar_t *a3) {
  return a0.widen(a1, a2, a3);
}

char f25(const std::ctype_byname<wchar_t> &a0, wchar_t a1, char a2) {
  return a0.narrow(a1, a2);
}

const wchar_t *f26(const std::ctype_byname<wchar_t> &a0, const wchar_t *a1,
                   const wchar_t *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

template <typename T1> using t3 = std::ctype_byname<T1>;

template <typename T1> using t4 = typename std::ctype_byname<T1>::mask;

template <typename T1>
std::ctype_byname<T1> f27(const char *a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1>
std::ctype_byname<T1> f28(const std::string &a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1>
bool f29(const std::ctype_byname<T1> &a0, unsigned short a1, T1 a2) {
  return a0.is(a1, std::move(a2));
}

template <typename T1>
T1 const *f30(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              unsigned short *a3) {
  return a0.is(a1, a2, a3);
}

template <typename T1>
T1 const *f31(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_is(a1, a2, a3);
}

template <typename T1>
T1 const *f32(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_not(a1, a2, a3);
}

template <typename T1> T1 f33(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.toupper(std::move(a1));
}

template <typename T1>
T1 const *f34(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.toupper(a1, a2);
}

template <typename T1> T1 f35(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.tolower(std::move(a1));
}

template <typename T1>
T1 const *f36(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.tolower(a1, a2);
}

template <typename T1> T1 f37(const std::ctype_byname<T1> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1>
const char *f38(const std::ctype_byname<T1> &a0, const char *a1, const char *a2,
                T1 *a3) {
  return a0.widen(a1, a2, a3);
}

template <typename T1>
char f39(const std::ctype_byname<T1> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1>
T1 const *f40(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}
