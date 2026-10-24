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

const wchar_t *f118(const wchar_t *a0, std::size_t a1, const wchar_t &a2) {
  return std::char_traits<wchar_t>::find(a0, a1, a2);
}

void f119(wchar_t &a0, const wchar_t &a1) {
  return std::char_traits<wchar_t>::assign(a0, a1);
}

bool f120(wchar_t a0, wchar_t a1) {
  return std::char_traits<wchar_t>::eq(a0, a1);
}

bool f121(wchar_t a0, wchar_t a1) {
  return std::char_traits<wchar_t>::lt(a0, a1);
}

wchar_t *f122(wchar_t *a0, const wchar_t *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::move(a0, a1, a2);
}

wchar_t *f123(wchar_t *a0, const wchar_t *a1, std::size_t a2) {
  return std::char_traits<wchar_t>::copy(a0, a1, a2);
}

wchar_t *f124(wchar_t *a0, std::size_t a1, wchar_t a2) {
  return std::char_traits<wchar_t>::assign(a0, a1, a2);
}

wchar_t f125(unsigned int a0) {
  return std::char_traits<wchar_t>::to_char_type(a0);
}

unsigned int f126(wchar_t a0) {
  return std::char_traits<wchar_t>::to_int_type(a0);
}

bool f127(unsigned int a0, unsigned int a1) {
  return std::char_traits<wchar_t>::eq_int_type(a0, a1);
}

unsigned int f128() { return std::char_traits<wchar_t>::eof(); }

unsigned int f129(unsigned int a0) {
  return std::char_traits<wchar_t>::not_eof(a0);
}

int f130(const char16_t *a0, const char16_t *a1, std::size_t a2) {
  return std::char_traits<char16_t>::compare(a0, a1, a2);
}

std::size_t f131(const char16_t *a0) {
  return std::char_traits<char16_t>::length(a0);
}

const char16_t *f132(const char16_t *a0, std::size_t a1, const char16_t &a2) {
  return std::char_traits<char16_t>::find(a0, a1, a2);
}

void f133(char16_t &a0, const char16_t &a1) {
  return std::char_traits<char16_t>::assign(a0, a1);
}

bool f134(char16_t a0, char16_t a1) {
  return std::char_traits<char16_t>::eq(a0, a1);
}

bool f135(char16_t a0, char16_t a1) {
  return std::char_traits<char16_t>::lt(a0, a1);
}

char16_t *f136(char16_t *a0, const char16_t *a1, std::size_t a2) {
  return std::char_traits<char16_t>::move(a0, a1, a2);
}

char16_t *f137(char16_t *a0, const char16_t *a1, std::size_t a2) {
  return std::char_traits<char16_t>::copy(a0, a1, a2);
}

char16_t *f138(char16_t *a0, std::size_t a1, char16_t a2) {
  return std::char_traits<char16_t>::assign(a0, a1, a2);
}

char16_t f139(unsigned short a0) {
  return std::char_traits<char16_t>::to_char_type(a0);
}

unsigned short f140(char16_t a0) {
  return std::char_traits<char16_t>::to_int_type(a0);
}

bool f141(unsigned short a0, unsigned short a1) {
  return std::char_traits<char16_t>::eq_int_type(a0, a1);
}

unsigned short f142() { return std::char_traits<char16_t>::eof(); }

unsigned short f143(unsigned short a0) {
  return std::char_traits<char16_t>::not_eof(a0);
}

int f144(const char32_t *a0, const char32_t *a1, std::size_t a2) {
  return std::char_traits<char32_t>::compare(a0, a1, a2);
}

std::size_t f145(const char32_t *a0) {
  return std::char_traits<char32_t>::length(a0);
}

const char32_t *f146(const char32_t *a0, std::size_t a1, const char32_t &a2) {
  return std::char_traits<char32_t>::find(a0, a1, a2);
}

void f147(char32_t &a0, const char32_t &a1) {
  return std::char_traits<char32_t>::assign(a0, a1);
}

bool f148(char32_t a0, char32_t a1) {
  return std::char_traits<char32_t>::eq(a0, a1);
}

bool f149(char32_t a0, char32_t a1) {
  return std::char_traits<char32_t>::lt(a0, a1);
}

char32_t *f150(char32_t *a0, const char32_t *a1, std::size_t a2) {
  return std::char_traits<char32_t>::move(a0, a1, a2);
}

char32_t *f151(char32_t *a0, const char32_t *a1, std::size_t a2) {
  return std::char_traits<char32_t>::copy(a0, a1, a2);
}

char32_t *f152(char32_t *a0, std::size_t a1, char32_t a2) {
  return std::char_traits<char32_t>::assign(a0, a1, a2);
}

char32_t f153(unsigned int a0) {
  return std::char_traits<char32_t>::to_char_type(a0);
}

unsigned int f154(char32_t a0) {
  return std::char_traits<char32_t>::to_int_type(a0);
}

bool f155(unsigned int a0, unsigned int a1) {
  return std::char_traits<char32_t>::eq_int_type(a0, a1);
}

unsigned int f156() { return std::char_traits<char32_t>::eof(); }

unsigned int f157(unsigned int a0) {
  return std::char_traits<char32_t>::not_eof(a0);
}
