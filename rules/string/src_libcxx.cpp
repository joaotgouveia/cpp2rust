#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f19(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f20(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f21(std::basic_string<T1, T2, T3> &a0, std::basic_string<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f22(std::basic_string<T1, T2, T3> &&a0,
                                  const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f23(std::basic_string<T1, T2, T3> &&a0,
                                  const T1 *a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f24(std::basic_string<T1, T2, T3> &&a0,
                                  std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f25(std::basic_string<T1, T2, T3> &&a0, T1 a1) {
  return std::operator+ <T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f26(const std::basic_string<T1, T2, T3> &a0,
                                  std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f27(const std::basic_string<T1, T2, T3> &a0,
                                  const T1 *a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f28(const std::basic_string<T1, T2, T3> &a0,
                                  const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f29(const std::basic_string<T1, T2, T3> &a0,
                                  T1 a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f30(const T1 *a0,
                                  std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f31(const T1 *a0,
                                  const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f32(T1 a0, std::basic_string<T1, T2, T3> &&a1) {
  return std::operator+ <T1, T2, T3>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f33(T1 a0,
                                  const std::basic_string<T1, T2, T3> &a1) {
  return std::operator+ <T1, T2, T3>(a0, a1);
}

double f34(const std::string &a0, std::size_t *a1) { return std::stod(a0, a1); }

double f35(const std::wstring &a0, std::size_t *a1) {
  return std::stod(a0, a1);
}

float f36(const std::string &a0, std::size_t *a1) { return std::stof(a0, a1); }

float f37(const std::wstring &a0, std::size_t *a1) { return std::stof(a0, a1); }

int f38(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

int f39(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoi(a0, a1, a2);
}

long f40(const std::string &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long f41(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stol(a0, a1, a2);
}

long double f42(const std::string &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long double f43(const std::wstring &a0, std::size_t *a1) {
  return std::stold(a0, a1);
}

long long f44(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

long long f45(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoll(a0, a1, a2);
}

unsigned long f46(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long f47(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoul(a0, a1, a2);
}

unsigned long long f48(const std::string &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

unsigned long long f49(const std::wstring &a0, std::size_t *a1, int a2) {
  return std::stoull(a0, a1, a2);
}

using t7 = std::string;

std::string f50(long double a0) { return std::to_string(a0); }

std::string f51(int a0) { return std::to_string(a0); }

std::string f52(long long a0) { return std::to_string(a0); }

std::string f53(long a0) { return std::to_string(a0); }

std::string f54(double a0) { return std::to_string(a0); }

std::string f55(float a0) { return std::to_string(a0); }

std::string f56(unsigned int a0) { return std::to_string(a0); }

std::string f57(unsigned long long a0) { return std::to_string(a0); }

std::string f58(unsigned long a0) { return std::to_string(a0); }

std::wstring f59(long double a0) { return std::to_wstring(a0); }

std::wstring f60(int a0) { return std::to_wstring(a0); }

std::wstring f61(long long a0) { return std::to_wstring(a0); }

std::wstring f62(long a0) { return std::to_wstring(a0); }

std::wstring f63(double a0) { return std::to_wstring(a0); }

std::wstring f64(float a0) { return std::to_wstring(a0); }

std::wstring f65(unsigned int a0) { return std::to_wstring(a0); }

std::wstring f66(unsigned long long a0) { return std::to_wstring(a0); }

std::wstring f67(unsigned long a0) { return std::to_wstring(a0); }

using t8 = std::u16string;

using t9 = std::u32string;

using t10 = std::wstring;
