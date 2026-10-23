#include <algorithm>
#include <codecvt>
#include <cwchar>
#include <ios>
#include <locale>
#include <utility>

int f46(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f47(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}
