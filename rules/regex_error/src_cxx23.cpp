#include <regex>
#include <utility>

std::regex_constants::error_type f1() {
  return std::regex_constants::error_backref;
}

std::regex_constants::error_type f2() {
  return std::regex_constants::error_badbrace;
}

std::regex_constants::error_type f3() {
  return std::regex_constants::error_badrepeat;
}

std::regex_constants::error_type f4() {
  return std::regex_constants::error_brace;
}

std::regex_constants::error_type f5() {
  return std::regex_constants::error_brack;
}

std::regex_constants::error_type f6() {
  return std::regex_constants::error_collate;
}

std::regex_constants::error_type f7() {
  return std::regex_constants::error_complexity;
}

std::regex_constants::error_type f8() {
  return std::regex_constants::error_ctype;
}

std::regex_constants::error_type f9() {
  return std::regex_constants::error_escape;
}

std::regex_constants::error_type f10() {
  return std::regex_constants::error_paren;
}

std::regex_constants::error_type f11() {
  return std::regex_constants::error_range;
}

std::regex_constants::error_type f12() {
  return std::regex_constants::error_space;
}

std::regex_constants::error_type f13() {
  return std::regex_constants::error_stack;
}

using t1 = std::regex_constants::error_type;

using t2 = std::regex_error;

std::regex_error f14(std::regex_constants::error_type a0) {
  return std::regex_error(a0);
}

std::regex_constants::error_type f15(const std::regex_error &a0) {
  return a0.code();
}

std::regex_error f16(const std::regex_error &a0) {
  return std::regex_error(a0);
}

std::regex_error &f17(std::regex_error &a0, const std::regex_error &a1) {
  return a0.operator=(a1);
}

const char *f18(const std::regex_error &a0) { return a0.what(); }
