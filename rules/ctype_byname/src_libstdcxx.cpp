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

const char *f16(const std::ctype_byname<wchar_t> &a0, const char *a1,
                const char *a2, wchar_t *a3) {
  return a0.widen(a1, a2, a3);
}

char f17(const std::ctype_byname<wchar_t> &a0, wchar_t a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1> using t3 = std::ctype_byname<T1>;

template <typename T1> using t4 = typename std::ctype_byname<T1>::mask;

template <typename T1>
std::ctype_byname<T1> f18(const char *a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1>
std::ctype_byname<T1> f19(const std::string &a0, std::size_t a1) {
  return std::ctype_byname<T1>(a0, a1);
}

template <typename T1>
bool f20(const std::ctype_byname<T1> &a0, unsigned short a1, T1 a2) {
  return a0.is(a1, a2);
}

template <typename T1>
T1 const *f21(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              unsigned short *a3) {
  return a0.is(a1, a2, a3);
}

template <typename T1>
T1 const *f22(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_is(a1, a2, a3);
}

template <typename T1>
T1 const *f23(const std::ctype_byname<T1> &a0, unsigned short a1, T1 const *a2,
              T1 const *a3) {
  return a0.scan_not(a1, a2, a3);
}

template <typename T1> T1 f24(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.toupper(a1);
}

template <typename T1>
T1 const *f25(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.toupper(a1, a2);
}

template <typename T1> T1 f26(const std::ctype_byname<T1> &a0, T1 a1) {
  return a0.tolower(a1);
}

template <typename T1>
T1 const *f27(const std::ctype_byname<T1> &a0, T1 *a1, T1 const *a2) {
  return a0.tolower(a1, a2);
}

template <typename T1> T1 f28(const std::ctype_byname<T1> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1>
const char *f29(const std::ctype_byname<T1> &a0, const char *a1, const char *a2,
                T1 *a3) {
  return a0.widen(a1, a2, a3);
}

template <typename T1>
char f30(const std::ctype_byname<T1> &a0, T1 a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1>
T1 const *f31(const std::ctype_byname<T1> &a0, T1 const *a1, T1 const *a2,
              char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}
