#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

int f158(const char8_t *a0, const char8_t *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f159(const char8_t *a0) {
  return std::char_traits<char8_t>::length(a0);
}

const char8_t *f160(const char8_t *a0, std::size_t a1, const char8_t &a2) {
  return std::char_traits<char8_t>::find(a0, a1, a2);
}

void f161(char8_t &a0, const char8_t &a1) {
  return std::char_traits<char8_t>::assign(a0, a1);
}

bool f162(char8_t a0, char8_t a1) {
  return std::char_traits<char8_t>::eq(a0, a1);
}

bool f163(char8_t a0, char8_t a1) {
  return std::char_traits<char8_t>::lt(a0, a1);
}

char8_t *f164(char8_t *a0, const char8_t *a1, std::size_t a2) {
  return std::char_traits<char8_t>::move(a0, a1, a2);
}

char8_t *f165(char8_t *a0, const char8_t *a1, std::size_t a2) {
  return std::char_traits<char8_t>::copy(a0, a1, a2);
}

char8_t *f166(char8_t *a0, std::size_t a1, char8_t a2) {
  return std::char_traits<char8_t>::assign(a0, a1, a2);
}

char8_t f167(unsigned int a0) {
  return std::char_traits<char8_t>::to_char_type(a0);
}

unsigned int f168(char8_t a0) {
  return std::char_traits<char8_t>::to_int_type(a0);
}

bool f169(unsigned int a0, unsigned int a1) {
  return std::char_traits<char8_t>::eq_int_type(a0, a1);
}

unsigned int f170() { return std::char_traits<char8_t>::eof(); }

unsigned int f171(unsigned int a0) {
  return std::char_traits<char8_t>::not_eof(a0);
}
