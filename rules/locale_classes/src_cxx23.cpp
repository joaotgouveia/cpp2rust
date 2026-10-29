#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1> bool f1(const std::locale &a0) {
  return std::has_facet<T1>(a0);
}

using t1 = std::locale;

using t2 = std::locale::category;

using t3 = std::locale::facet;

using t4 = std::locale::id;

std::locale f2() { return std::locale(); }

std::locale f3(const std::locale &a0) { return std::locale(a0); }

std::locale f4(const char *a0) { return std::locale(a0); }

std::locale f5(const std::locale &a0, const char *a1,
               std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f6(const std::string &a0) { return std::locale(a0); }

std::locale f7(const std::locale &a0, const std::string &a1,
               std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f8(const std::locale &a0, const std::locale &a1,
               std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

const std::locale &f9(std::locale &a0, const std::locale &a1) {
  return a0.operator=(a1);
}

std::string f10(const std::locale &a0) { return a0.name(); }

bool f11(const std::locale &a0, const std::locale &a1) {
  return a0.operator==(a1);
}

std::locale f12(const std::locale &a0) { return std::locale::global(a0); }

const std::locale &f13() { return std::locale::classic(); }

template <typename T1> std::locale f14(const std::locale &a0, T1 *a1) {
  return std::locale(a0, a1);
}

template <typename T1>
std::locale f15(const std::locale &a0, const std::locale &a1) {
  return a0.template combine<T1>(a1);
}

template <typename T1, typename T2, typename T3>
bool f16(const std::locale &a0, const std::basic_string<T1, T2, T3> &a1,
         const std::basic_string<T1, T2, T3> &a2) {
  return a0.template operator()<T1, T2, T3>(a1, a2);
}

std::locale::category f17() { return std::locale::none; }

std::locale::category f18() { return std::locale::ctype; }

std::locale::category f19() { return std::locale::numeric; }

std::locale::category f20() { return std::locale::collate; }

std::locale::category f21() { return std::locale::time; }

std::locale::category f22() { return std::locale::monetary; }

std::locale::category f23() { return std::locale::messages; }

std::locale::category f24() { return std::locale::all; }

std::locale::id f25() { return {}; }

using t5 = std::locale::_Impl;

template <typename T1> const T1 &f26(const std::locale &a0) {
  return std::use_facet<T1>(a0);
}
