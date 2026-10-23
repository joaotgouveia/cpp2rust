#include <ios>
#include <string.h>
#include <utility>

using t13 = std::numpunct<char>;

using t14 = std::numpunct<char>::char_type;

using t15 = std::numpunct<char>::string_type;

std::numpunct<char>::char_type f87(const std::numpunct<char> &a0) {
  return a0.decimal_point();
}

std::numpunct<char>::char_type f88(const std::numpunct<char> &a0) {
  return a0.thousands_sep();
}

std::string f89(const std::numpunct<char> &a0) { return a0.grouping(); }

std::numpunct<char>::string_type f90(const std::numpunct<char> &a0) {
  return a0.truename();
}

std::numpunct<char>::string_type f91(const std::numpunct<char> &a0) {
  return a0.falsename();
}

long f92(const std::numpunct<char> &a0) { return a0.use_count(); }

const std::locale::id &f93() { return std::numpunct<char>::id; }

using t16 = std::numpunct<wchar_t>;

using t17 = std::numpunct<wchar_t>::char_type;

using t18 = std::numpunct<wchar_t>::string_type;

std::numpunct<wchar_t>::char_type f94(const std::numpunct<wchar_t> &a0) {
  return a0.decimal_point();
}

std::numpunct<wchar_t>::char_type f95(const std::numpunct<wchar_t> &a0) {
  return a0.thousands_sep();
}

std::string f96(const std::numpunct<wchar_t> &a0) { return a0.grouping(); }

std::numpunct<wchar_t>::string_type f97(const std::numpunct<wchar_t> &a0) {
  return a0.truename();
}

std::numpunct<wchar_t>::string_type f98(const std::numpunct<wchar_t> &a0) {
  return a0.falsename();
}

long f99(const std::numpunct<wchar_t> &a0) { return a0.use_count(); }

const std::locale::id &f100() { return std::numpunct<wchar_t>::id; }
