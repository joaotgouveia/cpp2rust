#include <compare>
#include <iosfwd>
#include <system_error>
#include <utility>

using t1 = std::error_category;

const char *f1(const std::error_category &a0) { return a0.name(); }

std::string f2(const std::error_category &a0, int a1) { return a0.message(a1); }

std::error_condition f3(const std::error_category &a0, int a1) {
  return a0.default_error_condition(a1);
}

bool f4(const std::error_category &a0, int a1, const std::error_condition &a2) {
  return a0.equivalent(a1, a2);
}

bool f5(const std::error_category &a0, const std::error_code &a1, int a2) {
  return a0.equivalent(a1, a2);
}

bool f6(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator==(a1);
}

std::strong_ordering f7(const std::error_category &a0,
                        const std::error_category &a1) {
  return a0.operator<=>(a1);
}

using t2 = std::error_code;

std::error_code f8() { return std::error_code(); }

std::error_code f9(int a0, const std::error_category &a1) {
  return std::error_code(a0, a1);
}

std::error_code f10(const std::error_code &a0) { return std::error_code(a0); }

std::error_code &f11(std::error_code &a0, const std::error_code &a1) {
  return a0.operator=(a1);
}

void f12(std::error_code &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f13(std::error_code &a0) { return a0.clear(); }

int f14(const std::error_code &a0) { return a0.value(); }

const std::error_category &f15(const std::error_code &a0) {
  return a0.category();
}

std::error_condition f16(const std::error_code &a0) {
  return a0.default_error_condition();
}

std::string f17(const std::error_code &a0) { return a0.message(); }

bool f18(const std::error_code &a0) { return static_cast<bool>(a0); }

template <typename T1>
std::error_code f19(T1 a0)
  requires(!(std::is_same<T1, const std::error_code &>::value))
{
  return std::error_code(std::move(a0));
}

using t3 = std::error_condition;

std::error_condition f20() { return std::error_condition(); }

std::error_condition f21(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

std::error_condition f22(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition &f23(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

void f24(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f25(std::error_condition &a0) { return a0.clear(); }

int f26(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f27(const std::error_condition &a0) {
  return a0.category();
}

std::string f28(const std::error_condition &a0) { return a0.message(); }

bool f29(const std::error_condition &a0) { return static_cast<bool>(a0); }

template <typename T1>
std::error_condition f30(T1 a0)
  requires(!(std::is_same<T1, const std::error_condition &>::value))
{
  return std::error_condition(std::move(a0));
}

const std::error_category &f31() { return std::generic_category(); }

template <typename T1> bool f32() { return std::is_error_code_enum_v<T1>; }

template <typename T1> bool f33() { return std::is_error_condition_enum_v<T1>; }

std::error_code f34(std::errc a0) { return std::make_error_code(a0); }

std::error_condition f35(std::errc a0) { return std::make_error_condition(a0); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f36(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

std::strong_ordering f37(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<=>(a0, a1);
}

std::strong_ordering f38(const std::error_condition &a0,
                         const std::error_condition &a1) {
  return std::operator<=>(a0, a1);
}

bool f39(const std::error_code &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

bool f40(const std::error_code &a0, const std::error_code &a1) {
  return std::operator==(a0, a1);
}

bool f41(const std::error_condition &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

const std::error_category &f42() { return std::system_category(); }

using t4 = std::system_error;

std::system_error f43(std::error_code a0) {
  return std::system_error(std::move(a0));
}

std::system_error f44(std::error_code a0, const std::string &a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f45(std::error_code a0, const char *a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f46(int a0, const std::error_category &a1, const char *a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f47(int a0, const std::error_category &a1) {
  return std::system_error(a0, a1);
}

std::system_error f48(int a0, const std::error_category &a1,
                      const std::string &a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f49(const std::system_error &a0) {
  return std::system_error(a0);
}

std::system_error &f50(std::system_error &a0, const std::system_error &a1) {
  return a0.operator=(a1);
}

const std::error_code &f51(const std::system_error &a0) { return a0.code(); }

const char *f52(const std::system_error &a0) { return a0.what(); }
