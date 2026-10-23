#include <algorithm>
#include <codecvt>
#include <cstddef>
#include <cwchar>
#include <ios>
#include <locale>
#include <utility>

using t18 = std::codecvt<char16_t, char8_t, std::mbstate_t>;

using t19 = std::codecvt<char16_t, char8_t, __mbstate_t>::intern_type;

using t20 = std::codecvt<char16_t, char8_t, __mbstate_t>::extern_type;

using t21 = std::codecvt<char16_t, char8_t, __mbstate_t>::state_type;

int f20(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f21(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f22(const std::codecvt<char16_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f23() {
  return std::codecvt<char16_t, char8_t, __mbstate_t>::id;
}

using t22 = std::codecvt<char32_t, char8_t, std::mbstate_t>;

using t23 = std::codecvt<char32_t, char8_t, __mbstate_t>::intern_type;

using t24 = std::codecvt<char32_t, char8_t, __mbstate_t>::extern_type;

using t25 = std::codecvt<char32_t, char8_t, __mbstate_t>::state_type;

int f24(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f25(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f26(const std::codecvt<char32_t, char8_t, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f27() {
  return std::codecvt<char32_t, char8_t, __mbstate_t>::id;
}
