#include <ios>
#include <string.h>
#include <utility>

using t4 = std::numpunct<char>;

using t5 = std::numpunct<char>::char_type;

using t6 = std::numpunct<char>::string_type;

std::numpunct<char>::char_type f8(const std::numpunct<char> &a0) {
  return a0.decimal_point();
}

std::numpunct<char>::char_type f9(const std::numpunct<char> &a0) {
  return a0.thousands_sep();
}

std::string f10(const std::numpunct<char> &a0) { return a0.grouping(); }

std::numpunct<char>::string_type f11(const std::numpunct<char> &a0) {
  return a0.truename();
}

std::numpunct<char>::string_type f12(const std::numpunct<char> &a0) {
  return a0.falsename();
}

long f13(const std::numpunct<char> &a0) { return a0.use_count(); }

const std::locale::id &f14() { return std::numpunct<char>::id; }

using t7 = std::numpunct<wchar_t>;

using t8 = std::numpunct<wchar_t>::char_type;

using t9 = std::numpunct<wchar_t>::string_type;

std::numpunct<wchar_t>::char_type f15(const std::numpunct<wchar_t> &a0) {
  return a0.decimal_point();
}

std::numpunct<wchar_t>::char_type f16(const std::numpunct<wchar_t> &a0) {
  return a0.thousands_sep();
}

std::string f17(const std::numpunct<wchar_t> &a0) { return a0.grouping(); }

std::numpunct<wchar_t>::string_type f18(const std::numpunct<wchar_t> &a0) {
  return a0.truename();
}

std::numpunct<wchar_t>::string_type f19(const std::numpunct<wchar_t> &a0) {
  return a0.falsename();
}

long f20(const std::numpunct<wchar_t> &a0) { return a0.use_count(); }

const std::locale::id &f21() { return std::numpunct<wchar_t>::id; }
