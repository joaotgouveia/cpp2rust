#include "../common/init.h"
#include "../common/traits.h"
#include <charconv>
#include <utility>

std::chars_format f1(std::chars_format a0, std::chars_format a1) {
  return a0 & a1;
}

std::chars_format &f2(std::chars_format &a0, std::chars_format a1) {
  return std::operator&=(a0, a1);
}

std::chars_format f3(std::chars_format a0, std::chars_format a1) {
  return a0 ^ a1;
}

std::chars_format &f4(std::chars_format &a0, std::chars_format a1) {
  return std::operator^=(a0, a1);
}

std::chars_format f5(std::chars_format a0, std::chars_format a1) {
  return a0 | a1;
}

std::chars_format &f6(std::chars_format &a0, std::chars_format a1) {
  return std::operator|=(a0, a1);
}

std::chars_format f7(std::chars_format a0) { return ~a0; }

using t1 = std::chars_format;

std::chars_format f8() { return std::chars_format::scientific; }

std::chars_format f9() { return std::chars_format::fixed; }

std::chars_format f10() { return std::chars_format::hex; }

std::chars_format f11() { return std::chars_format::general; }
