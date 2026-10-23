#include <algorithm>
#include <ios>
#include <string>
#include <utility>

bool f13(const std::ctype<char> &a0, std::ctype_base::mask a1, char a2) {
  return a0.is(a1, a2);
}

const char *f14(const std::ctype<char> &a0, const char *a1, const char *a2,
                std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const char *f15(const std::ctype<char> &a0, std::ctype_base::mask a1,
                const char *a2, const char *a3) {
  return a0.scan_is(a1, a2, a3);
}

const char *f16(const std::ctype<char> &a0, std::ctype_base::mask a1,
                const char *a2, const char *a3) {
  return a0.scan_not(a1, a2, a3);
}

const std::ctype<char>::char_type *f17(const std::ctype<char> &a0,
                                       const std::ctype<char>::char_type *a1,
                                       const std::ctype<char>::char_type *a2,
                                       char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

bool f18(const std::ctype<wchar_t> &a0, std::ctype_base::mask a1, wchar_t a2) {
  return a0.is(a1, a2);
}

const char *f19(const std::ctype<wchar_t> &a0, const char *a1, const char *a2,
                wchar_t *a3) {
  return a0.widen(a1, a2, a3);
}

char f20(const std::ctype<wchar_t> &a0, wchar_t a1, char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1> using t5 = std::ctype<T1>;

template <typename T1> using t6 = typename std::ctype<T1>::char_type;

template <typename T1> using t7 = typename std::ctype<T1>::mask;

template <typename T1> std::ctype<T1> f21(std::size_t a0) {
  return std::ctype<T1>(a0);
}

template <typename T1>
bool f22(const std::ctype<T1> &a0, typename std::ctype<T1>::mask a1,
         typename std::ctype<T1>::char_type a2) {
  return a0.is(a1, a2);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f23(const std::ctype<T1> &a0, const typename std::ctype<T1>::char_type *a1,
    const typename std::ctype<T1>::char_type *a2,
    typename std::ctype<T1>::mask *a3) {
  return a0.is(a1, a2, a3);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f24(const std::ctype<T1> &a0, typename std::ctype<T1>::mask a1,
    const typename std::ctype<T1>::char_type *a2,
    const typename std::ctype<T1>::char_type *a3) {
  return a0.scan_is(a1, a2, a3);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f25(const std::ctype<T1> &a0, typename std::ctype<T1>::mask a1,
    const typename std::ctype<T1>::char_type *a2,
    const typename std::ctype<T1>::char_type *a3) {
  return a0.scan_not(a1, a2, a3);
}

template <typename T1>
typename std::ctype<T1>::char_type f26(const std::ctype<T1> &a0,
                                       typename std::ctype<T1>::char_type a1) {
  return a0.toupper(a1);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f27(const std::ctype<T1> &a0, typename std::ctype<T1>::char_type *a1,
    const typename std::ctype<T1>::char_type *a2) {
  return a0.toupper(a1, a2);
}

template <typename T1>
typename std::ctype<T1>::char_type f28(const std::ctype<T1> &a0,
                                       typename std::ctype<T1>::char_type a1) {
  return a0.tolower(a1);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f29(const std::ctype<T1> &a0, typename std::ctype<T1>::char_type *a1,
    const typename std::ctype<T1>::char_type *a2) {
  return a0.tolower(a1, a2);
}

template <typename T1>
typename std::ctype<T1>::char_type f30(const std::ctype<T1> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1>
const char *f31(const std::ctype<T1> &a0, const char *a1, const char *a2,
                typename std::ctype<T1>::char_type *a3) {
  return a0.widen(a1, a2, a3);
}

template <typename T1>
char f32(const std::ctype<T1> &a0, typename std::ctype<T1>::char_type a1,
         char a2) {
  return a0.narrow(a1, a2);
}

template <typename T1>
const typename std::ctype<T1>::char_type *
f33(const std::ctype<T1> &a0, const typename std::ctype<T1>::char_type *a1,
    const typename std::ctype<T1>::char_type *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

template <typename T1> const std::locale::id &f34() {
  return std::ctype<T1>::id;
}
