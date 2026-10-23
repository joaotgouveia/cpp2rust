#include <ios>
#include <utility>

bool f32(const std::ctype_byname<char> &a0, std::ctype_base::mask a1,
         std::ctype<char>::char_type a2) {
  return a0.is(a1, a2);
}

const std::ctype<char>::char_type *f33(const std::ctype_byname<char> &a0,
                                       const std::ctype<char>::char_type *a1,
                                       const std::ctype<char>::char_type *a2,
                                       std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const std::ctype<char>::char_type *f34(const std::ctype_byname<char> &a0,
                                       std::ctype_base::mask a1,
                                       const std::ctype<char>::char_type *a2,
                                       const std::ctype<char>::char_type *a3) {
  return a0.scan_is(a1, a2, a3);
}

const std::ctype<char>::char_type *f35(const std::ctype_byname<char> &a0,
                                       std::ctype_base::mask a1,
                                       const std::ctype<char>::char_type *a2,
                                       const std::ctype<char>::char_type *a3) {
  return a0.scan_not(a1, a2, a3);
}

const char *f36(const std::ctype_byname<char> &a0,
                const std::ctype<char>::char_type *a1,
                const std::ctype<char>::char_type *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

long f37(const std::ctype_byname<char> &a0) { return a0.use_count(); }

bool f38(const std::ctype_byname<wchar_t> &a0, std::ctype_base::mask a1,
         std::ctype<wchar_t>::char_type a2) {
  return a0.is(a1, a2);
}

const std::ctype<wchar_t>::char_type *
f39(const std::ctype_byname<wchar_t> &a0,
    const std::ctype<wchar_t>::char_type *a1,
    const std::ctype<wchar_t>::char_type *a2, std::ctype_base::mask *a3) {
  return a0.is(a1, a2, a3);
}

const std::ctype<wchar_t>::char_type *
f40(const std::ctype_byname<wchar_t> &a0, std::ctype_base::mask a1,
    const std::ctype<wchar_t>::char_type *a2,
    const std::ctype<wchar_t>::char_type *a3) {
  return a0.scan_is(a1, a2, a3);
}

const std::ctype<wchar_t>::char_type *
f41(const std::ctype_byname<wchar_t> &a0, std::ctype_base::mask a1,
    const std::ctype<wchar_t>::char_type *a2,
    const std::ctype<wchar_t>::char_type *a3) {
  return a0.scan_not(a1, a2, a3);
}

std::ctype<wchar_t>::char_type f42(const std::ctype_byname<wchar_t> &a0,
                                   std::ctype<wchar_t>::char_type a1) {
  return a0.toupper(a1);
}

const std::ctype<wchar_t>::char_type *
f43(const std::ctype_byname<wchar_t> &a0, std::ctype<wchar_t>::char_type *a1,
    const std::ctype<wchar_t>::char_type *a2) {
  return a0.toupper(a1, a2);
}

std::ctype<wchar_t>::char_type f44(const std::ctype_byname<wchar_t> &a0,
                                   std::ctype<wchar_t>::char_type a1) {
  return a0.tolower(a1);
}

const std::ctype<wchar_t>::char_type *
f45(const std::ctype_byname<wchar_t> &a0, std::ctype<wchar_t>::char_type *a1,
    const std::ctype<wchar_t>::char_type *a2) {
  return a0.tolower(a1, a2);
}

std::ctype<wchar_t>::char_type f46(const std::ctype_byname<wchar_t> &a0,
                                   char a1) {
  return a0.widen(a1);
}

const char *f47(const std::ctype_byname<wchar_t> &a0, const char *a1,
                const char *a2, std::ctype<wchar_t>::char_type *a3) {
  return a0.widen(a1, a2, a3);
}

char f48(const std::ctype_byname<wchar_t> &a0,
         std::ctype<wchar_t>::char_type a1, char a2) {
  return a0.narrow(a1, a2);
}

const std::ctype<wchar_t>::char_type *
f49(const std::ctype_byname<wchar_t> &a0,
    const std::ctype<wchar_t>::char_type *a1,
    const std::ctype<wchar_t>::char_type *a2, char a3, char *a4) {
  return a0.narrow(a1, a2, a3, a4);
}

long f50(const std::ctype_byname<wchar_t> &a0) { return a0.use_count(); }
