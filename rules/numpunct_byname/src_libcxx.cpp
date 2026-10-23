#include <ios>
#include <string.h>
#include <utility>

using t4 = std::numpunct_byname<char>;

using t5 = std::numpunct_byname<char>::char_type;

using t6 = std::numpunct_byname<char>::string_type;

std::numpunct<char>::char_type f8(const std::numpunct_byname<char> &a0) {
  return a0.decimal_point();
}

std::numpunct<char>::char_type f9(const std::numpunct_byname<char> &a0) {
  return a0.thousands_sep();
}

std::string f10(const std::numpunct_byname<char> &a0) { return a0.grouping(); }

std::numpunct<char>::string_type f11(const std::numpunct_byname<char> &a0) {
  return a0.truename();
}

std::numpunct<char>::string_type f12(const std::numpunct_byname<char> &a0) {
  return a0.falsename();
}

long f13(const std::numpunct_byname<char> &a0) { return a0.use_count(); }

using t7 = std::numpunct_byname<wchar_t>;

using t8 = std::numpunct_byname<wchar_t>::char_type;

using t9 = std::numpunct_byname<wchar_t>::string_type;

std::numpunct<wchar_t>::char_type f14(const std::numpunct_byname<wchar_t> &a0) {
  return a0.decimal_point();
}

std::numpunct<wchar_t>::char_type f15(const std::numpunct_byname<wchar_t> &a0) {
  return a0.thousands_sep();
}

std::string f16(const std::numpunct_byname<wchar_t> &a0) {
  return a0.grouping();
}

std::numpunct<wchar_t>::string_type
f17(const std::numpunct_byname<wchar_t> &a0) {
  return a0.truename();
}

std::numpunct<wchar_t>::string_type
f18(const std::numpunct_byname<wchar_t> &a0) {
  return a0.falsename();
}

long f19(const std::numpunct_byname<wchar_t> &a0) { return a0.use_count(); }
