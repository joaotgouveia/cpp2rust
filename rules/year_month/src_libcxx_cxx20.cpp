#include <chrono>
#include <compare>
#include <utility>

std::chrono::year_month f1(const std::chrono::months &a0,
                           const std::chrono::year_month &a1) {
  return std::chrono::operator+(a0, a1);
}

std::chrono::year_month f2(const std::chrono::years &a0,
                           const std::chrono::year_month &a1) {
  return std::chrono::operator+(a0, a1);
}

std::chrono::year_month f3(const std::chrono::year_month &a0,
                           const std::chrono::months &a1) {
  return std::chrono::operator+(a0, a1);
}

std::chrono::year_month f4(const std::chrono::year_month &a0,
                           const std::chrono::years &a1) {
  return std::chrono::operator+(a0, a1);
}

std::chrono::year_month f5(const std::chrono::year_month &a0,
                           const std::chrono::months &a1) {
  return std::chrono::operator-(a0, a1);
}

std::chrono::year_month f6(const std::chrono::year_month &a0,
                           const std::chrono::years &a1) {
  return std::chrono::operator-(a0, a1);
}

std::chrono::months f7(const std::chrono::year_month &a0,
                       const std::chrono::year_month &a1) {
  return std::chrono::operator-(a0, a1);
}

std::chrono::year_month f8(const std::chrono::year &a0,
                           const std::chrono::month &a1) {
  return std::chrono::operator/(a0, a1);
}

std::chrono::year_month f9(const std::chrono::year &a0, int a1) {
  return std::chrono::operator/(a0, a1);
}

std::strong_ordering f10(const std::chrono::year_month &a0,
                         const std::chrono::year_month &a1) {
  return std::chrono::operator<=>(a0, a1);
}

bool f11(const std::chrono::year_month &a0, const std::chrono::year_month &a1) {
  return std::chrono::operator==(a0, a1);
}

using t1 = std::chrono::year_month;

std::chrono::year_month f12() { return std::chrono::year_month(); }

std::chrono::year_month f13(const std::chrono::year &a0,
                            const std::chrono::month &a1) {
  return std::chrono::year_month(a0, a1);
}

std::chrono::year f14(const std::chrono::year_month &a0) { return a0.year(); }

std::chrono::month f15(const std::chrono::year_month &a0) { return a0.month(); }

std::chrono::year_month &f16(std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f17(std::chrono::year_month &a0,
                             const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month &f18(std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month &f19(std::chrono::year_month &a0,
                             const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

bool f20(const std::chrono::year_month &a0) { return a0.ok(); }

std::chrono::year_month f21(const std::chrono::year_month &a0) {
  return std::chrono::year_month(a0);
}

std::chrono::year_month f22(std::chrono::year_month &&a0) {
  return std::chrono::year_month(std::move(a0));
}

std::chrono::year_month &f23(std::chrono::year_month &a0,
                             const std::chrono::year_month &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month &f24(std::chrono::year_month &a0,
                             std::chrono::year_month &&a1) {
  return a0.operator=(std::move(a1));
}
