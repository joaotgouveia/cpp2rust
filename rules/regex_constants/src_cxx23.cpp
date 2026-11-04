#include <regex>
#include <utility>

std::regex_constants::syntax_option_type f1() {
  return std::regex_constants::ECMAScript;
}

std::regex_constants::syntax_option_type f2() {
  return std::regex_constants::awk;
}

std::regex_constants::syntax_option_type f3() {
  return std::regex_constants::basic;
}

std::regex_constants::syntax_option_type f4() {
  return std::regex_constants::collate;
}

std::regex_constants::syntax_option_type f5() {
  return std::regex_constants::egrep;
}

std::regex_constants::syntax_option_type f6() {
  return std::regex_constants::extended;
}

std::regex_constants::match_flag_type f7() {
  return std::regex_constants::format_default;
}

std::regex_constants::match_flag_type f8() {
  return std::regex_constants::format_first_only;
}

std::regex_constants::match_flag_type f9() {
  return std::regex_constants::format_no_copy;
}

std::regex_constants::match_flag_type f10() {
  return std::regex_constants::format_sed;
}

std::regex_constants::syntax_option_type f11() {
  return std::regex_constants::grep;
}

std::regex_constants::syntax_option_type f12() {
  return std::regex_constants::icase;
}

std::regex_constants::match_flag_type f13() {
  return std::regex_constants::match_any;
}

std::regex_constants::match_flag_type f14() {
  return std::regex_constants::match_continuous;
}

std::regex_constants::match_flag_type f15() {
  return std::regex_constants::match_default;
}

using t1 = std::regex_constants::match_flag_type;

std::regex_constants::match_flag_type f16() {
  return std::regex_constants::match_not_bol;
}

std::regex_constants::match_flag_type f17() {
  return std::regex_constants::match_not_bow;
}

std::regex_constants::match_flag_type f18() {
  return std::regex_constants::match_not_eol;
}

std::regex_constants::match_flag_type f19() {
  return std::regex_constants::match_not_eow;
}

std::regex_constants::match_flag_type f20() {
  return std::regex_constants::match_not_null;
}

std::regex_constants::match_flag_type f21() {
  return std::regex_constants::match_prev_avail;
}

std::regex_constants::syntax_option_type f22() {
  return std::regex_constants::multiline;
}

std::regex_constants::syntax_option_type f23() {
  return std::regex_constants::nosubs;
}

std::regex_constants::match_flag_type
f24(std::regex_constants::match_flag_type a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator&(a0, a1);
}

std::regex_constants::syntax_option_type
f25(std::regex_constants::syntax_option_type a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator&(a0, a1);
}

std::regex_constants::match_flag_type &
f26(std::regex_constants::match_flag_type &a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::syntax_option_type &
f27(std::regex_constants::syntax_option_type &a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::match_flag_type
f28(std::regex_constants::match_flag_type a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator^(a0, a1);
}

std::regex_constants::syntax_option_type
f29(std::regex_constants::syntax_option_type a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator^(a0, a1);
}

std::regex_constants::match_flag_type &
f30(std::regex_constants::match_flag_type &a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::syntax_option_type &
f31(std::regex_constants::syntax_option_type &a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::match_flag_type
f32(std::regex_constants::match_flag_type a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator|(a0, a1);
}

std::regex_constants::syntax_option_type
f33(std::regex_constants::syntax_option_type a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator|(a0, a1);
}

std::regex_constants::match_flag_type &
f34(std::regex_constants::match_flag_type &a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::syntax_option_type &
f35(std::regex_constants::syntax_option_type &a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::match_flag_type
f36(std::regex_constants::match_flag_type a0) {
  return std::regex_constants::operator~(a0);
}

std::regex_constants::syntax_option_type
f37(std::regex_constants::syntax_option_type a0) {
  return std::regex_constants::operator~(a0);
}

std::regex_constants::syntax_option_type f38() {
  return std::regex_constants::optimize;
}

using t2 = std::regex_constants::syntax_option_type;
