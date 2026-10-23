#include <regex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::regex_constants::match_flag_type;

std::regex_constants::match_flag_type
f1(std::regex_constants::match_flag_type a0,
   std::regex_constants::match_flag_type a1) {
  return a0 & a1;
}

std::regex_constants::syntax_option_type
f2(std::regex_constants::syntax_option_type a0,
   std::regex_constants::syntax_option_type a1) {
  return a0 & a1;
}

std::regex_constants::match_flag_type &
f3(std::regex_constants::match_flag_type &a0,
   std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::syntax_option_type &
f4(std::regex_constants::syntax_option_type &a0,
   std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator&=(a0, a1);
}

std::regex_constants::match_flag_type
f5(std::regex_constants::match_flag_type a0,
   std::regex_constants::match_flag_type a1) {
  return a0 ^ a1;
}

std::regex_constants::syntax_option_type
f6(std::regex_constants::syntax_option_type a0,
   std::regex_constants::syntax_option_type a1) {
  return a0 ^ a1;
}

std::regex_constants::match_flag_type &
f7(std::regex_constants::match_flag_type &a0,
   std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::syntax_option_type &
f8(std::regex_constants::syntax_option_type &a0,
   std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator^=(a0, a1);
}

std::regex_constants::match_flag_type
f9(std::regex_constants::match_flag_type a0,
   std::regex_constants::match_flag_type a1) {
  return a0 | a1;
}

std::regex_constants::syntax_option_type
f10(std::regex_constants::syntax_option_type a0,
    std::regex_constants::syntax_option_type a1) {
  return a0 | a1;
}

std::regex_constants::match_flag_type &
f11(std::regex_constants::match_flag_type &a0,
    std::regex_constants::match_flag_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::syntax_option_type &
f12(std::regex_constants::syntax_option_type &a0,
    std::regex_constants::syntax_option_type a1) {
  return std::regex_constants::operator|=(a0, a1);
}

std::regex_constants::match_flag_type
f13(std::regex_constants::match_flag_type a0) {
  return ~a0;
}

std::regex_constants::syntax_option_type
f14(std::regex_constants::syntax_option_type a0) {
  return ~a0;
}

using t2 = std::regex_constants::syntax_option_type;
