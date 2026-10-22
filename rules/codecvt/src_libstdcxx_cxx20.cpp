#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <codecvt>
#include <cwchar>
#include <ios>
#include <locale>
#include <utility>

using t25 = std::codecvt<char16_t, char8_t, std::mbstate_t>;

using t26 = std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type;

using t27 = std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type;

using t28 = std::codecvt<char16_t, char8_t, __mbstate_t>::state_type;

int f38(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f39(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f40(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f41(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f42() {
  return std::codecvt<char16_t, char8_t, __mbstate_t>::id;
}

using t29 = std::codecvt<char32_t, char8_t, std::mbstate_t>;

using t30 = std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type;

using t31 = std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type;

using t32 = std::codecvt<char32_t, char8_t, __mbstate_t>::state_type;

int f43(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f44(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f45(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0,
        __mbstate_t &a1, const char8_t *a2, const char8_t *a3, std::size_t a4) {
  return a0.length(a1, a2, a3, a4);
}

int f46(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f47() {
  return std::codecvt<char32_t, char8_t, __mbstate_t>::id;
}
