#include <chrono>
#include <compare>
#include <utility>

std::chrono::year f1(const std::chrono::years &a0,
                     const std::chrono::year &a1) {
  return a0 + a1;
}

std::chrono::year f2(const std::chrono::year &a0,
                     const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::year f3(const std::chrono::year &a0,
                     const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::years f4(const std::chrono::year &a0,
                      const std::chrono::year &a1) {
  return a0 - a1;
}

std::strong_ordering f5(const std::chrono::year &a0,
                        const std::chrono::year &a1) {
  return a0 <=> a1;
}

bool f6(const std::chrono::year &a0, const std::chrono::year &a1) {
  return a0 == a1;
}

using t1 = std::chrono::year;

std::chrono::year f7() { return std::chrono::year(); }

std::chrono::year f8(int a0) { return std::chrono::year(a0); }

std::chrono::year &f9(std::chrono::year &a0) { return a0.operator++(); }

std::chrono::year f10(std::chrono::year &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::year &f11(std::chrono::year &a0) { return a0.operator--(); }

std::chrono::year f12(std::chrono::year &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::year &f13(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year &f14(std::chrono::year &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f15(const std::chrono::year &a0) { return a0.operator+(); }

std::chrono::year f16(const std::chrono::year &a0) { return a0.operator-(); }

bool f17(const std::chrono::year &a0) { return a0.is_leap(); }

int f18(const std::chrono::year &a0) { return static_cast<int>(a0); }

bool f19(const std::chrono::year &a0) { return a0.ok(); }

std::chrono::year f20() { return std::chrono::year::min(); }

std::chrono::year f21() { return std::chrono::year::max(); }

std::chrono::year f22(const std::chrono::year &a0) {
  return std::chrono::year(a0);
}

std::chrono::year f23(std::chrono::year &&a0) {
  return std::chrono::year(std::move(a0));
}

std::chrono::year &f24(std::chrono::year &a0, const std::chrono::year &a1) {
  return a0.operator=(a1);
}

std::chrono::year &f25(std::chrono::year &a0, std::chrono::year &&a1) {
  return a0.operator=(std::move(a1));
}
