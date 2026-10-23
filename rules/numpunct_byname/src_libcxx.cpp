#include <ios>
#include <string.h>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t13 = std::numpunct_byname<char>;

using t14 = std::numpunct_byname<char>::char_type;

using t15 = std::numpunct_byname<char>::string_type;

std::numpunct<char>::char_type f87(const std::numpunct_byname<char> &a0) {
  return a0.decimal_point();
}

std::numpunct<char>::char_type f88(const std::numpunct_byname<char> &a0) {
  return a0.thousands_sep();
}

std::string f89(const std::numpunct_byname<char> &a0) { return a0.grouping(); }

std::numpunct<char>::string_type f90(const std::numpunct_byname<char> &a0) {
  return a0.truename();
}

std::numpunct<char>::string_type f91(const std::numpunct_byname<char> &a0) {
  return a0.falsename();
}

long f92(const std::numpunct_byname<char> &a0) { return a0.use_count(); }

using t16 = std::numpunct_byname<wchar_t>;

using t17 = std::numpunct_byname<wchar_t>::char_type;

using t18 = std::numpunct_byname<wchar_t>::string_type;

std::numpunct<wchar_t>::char_type f93(const std::numpunct_byname<wchar_t> &a0) {
  return a0.decimal_point();
}

std::numpunct<wchar_t>::char_type f94(const std::numpunct_byname<wchar_t> &a0) {
  return a0.thousands_sep();
}

std::string f95(const std::numpunct_byname<wchar_t> &a0) {
  return a0.grouping();
}

std::numpunct<wchar_t>::string_type
f96(const std::numpunct_byname<wchar_t> &a0) {
  return a0.truename();
}

std::numpunct<wchar_t>::string_type
f97(const std::numpunct_byname<wchar_t> &a0) {
  return a0.falsename();
}

long f98(const std::numpunct_byname<wchar_t> &a0) { return a0.use_count(); }
