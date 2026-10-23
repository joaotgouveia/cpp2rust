#include <algorithm>
#include <codecvt>
#include <cstddef>
#include <ios>
#include <string>
#include <utility>

using t1 = std::codecvt<char, char, std::mbstate_t>;

using t2 = std::codecvt<char, char, __mbstate_t>::intern_type;

using t3 = std::codecvt<char, char, __mbstate_t>::extern_type;

using t4 = std::codecvt<char, char, __mbstate_t>::state_type;

int f1(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f2(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f3(const std::codecvt<char, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f4() {
  return std::codecvt<char, char, __mbstate_t>::id;
}

using t5 = std::codecvt<wchar_t, char, std::mbstate_t>;

using t6 = std::codecvt<wchar_t, char, __mbstate_t>::intern_type;

using t7 = std::codecvt<wchar_t, char, __mbstate_t>::extern_type;

using t8 = std::codecvt<wchar_t, char, __mbstate_t>::state_type;

int f5(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f6(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f7(const std::codecvt<wchar_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f8() {
  return std::codecvt<wchar_t, char, __mbstate_t>::id;
}

using t9 = std::codecvt<char16_t, char, std::mbstate_t>;

using t10 = std::codecvt<char16_t, char, __mbstate_t>::intern_type;

using t11 = std::codecvt<char16_t, char, __mbstate_t>::extern_type;

using t12 = std::codecvt<char16_t, char, __mbstate_t>::state_type;

int f9(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f10(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f11(const std::codecvt<char16_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f12() {
  return std::codecvt<char16_t, char, __mbstate_t>::id;
}

using t13 = std::codecvt<char32_t, char, std::mbstate_t>;

using t14 = std::codecvt<char32_t, char, __mbstate_t>::intern_type;

using t15 = std::codecvt<char32_t, char, __mbstate_t>::extern_type;

using t16 = std::codecvt<char32_t, char, __mbstate_t>::state_type;

int f13(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.encoding();
}

bool f14(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.always_noconv();
}

int f15(const std::codecvt<char32_t, char, std::mbstate_t> &a0) {
  return a0.max_length();
}

const std::locale::id &f16() {
  return std::codecvt<char32_t, char, __mbstate_t>::id;
}

using t17 = std::codecvt_mode;

std::codecvt_mode f17() { return std::consume_header; }

std::codecvt_mode f18() { return std::generate_header; }

std::codecvt_mode f19() { return std::little_endian; }
