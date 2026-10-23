#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

bool f110(std::char_traits<char>::char_type a0,
          std::char_traits<char>::char_type a1) {
  return std::char_traits<char>::eq(a0, a1);
}

bool f111(std::char_traits<char>::char_type a0,
          std::char_traits<char>::char_type a1) {
  return std::char_traits<char>::lt(a0, a1);
}

std::char_traits<char>::int_type f112(std::char_traits<char>::int_type a0) {
  return std::char_traits<char>::not_eof(a0);
}

std::char_traits<char>::char_type f113(std::char_traits<char>::int_type a0) {
  return std::char_traits<char>::to_char_type(a0);
}

std::char_traits<char>::int_type f114(std::char_traits<char>::char_type a0) {
  return std::char_traits<char>::to_int_type(a0);
}

bool f115(std::char_traits<char>::int_type a0,
          std::char_traits<char>::int_type a1) {
  return std::char_traits<char>::eq_int_type(a0, a1);
}

int f116(const wchar_t *a0, const wchar_t *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::compare(a0, a1, a2);
}

std::size_t f117(const wchar_t *a0) {
  return std::char_traits<wchar_t>::length(a0);
}

int f118(const char16_t *a0, const char16_t *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f119(const char16_t *a0) {
  return std::char_traits<char16_t>::length(a0);
}

int f120(const char32_t *a0, const char32_t *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f121(const char32_t *a0) {
  return std::char_traits<char32_t>::length(a0);
}
