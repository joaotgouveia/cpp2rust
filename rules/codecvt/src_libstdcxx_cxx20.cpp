#include <algorithm>
#include <codecvt>
#include <cwchar>
#include <ios>
#include <locale>
#include <utility>

std::codecvt_base::result
f62(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char16_t *a2, const char16_t *a3, const char16_t *&a4, char8_t *a5,
    char8_t *a6, char8_t *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f63(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    char8_t *a2, char8_t *a3, char8_t *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f64(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char8_t *a2, const char8_t *a3, const char8_t *&a4, char16_t *a5,
    char16_t *a6, char16_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f65(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

std::codecvt_base::result
f66(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char32_t *a2, const char32_t *a3, const char32_t *&a4, char8_t *a5,
    char8_t *a6, char8_t *&a7) {
  return a0.out(a1, a2, a3, a4, a5, a6, a7);
}

std::codecvt_base::result
f67(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    char8_t *a2, char8_t *a3, char8_t *&a4) {
  return a0.unshift(a1, a2, a3, a4);
}

std::codecvt_base::result
f68(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0, __mbstate_t &a1,
    const char8_t *a2, const char8_t *a3, const char8_t *&a4, char32_t *a5,
    char32_t *a6, char32_t *&a7) {
  return a0.in(a1, a2, a3, a4, a5, a6, a7);
}

int f69(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}
