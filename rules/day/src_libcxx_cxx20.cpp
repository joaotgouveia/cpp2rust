#include <chrono>
#include <compare>
#include <utility>

using t1 = std::chrono::day;

std::chrono::day f1() { return std::chrono::day(); }

std::chrono::day f2(unsigned int a0) { return std::chrono::day(a0); }

std::chrono::day &f3(std::chrono::day &a0) { return a0.operator++(); }

std::chrono::day f4(std::chrono::day &a0, int a1) { return a0.operator++(a1); }

std::chrono::day &f5(std::chrono::day &a0) { return a0.operator--(); }

std::chrono::day f6(std::chrono::day &a0, int a1) { return a0.operator--(a1); }

std::chrono::day &f7(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::day &f8(std::chrono::day &a0, const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f9(const std::chrono::day &a0) {
  return static_cast<unsigned int>(a0);
}

bool f10(const std::chrono::day &a0) { return a0.ok(); }

std::chrono::day f11(const std::chrono::day &a0) {
  return std::chrono::day(a0);
}

std::chrono::day f12(std::chrono::day &&a0) {
  return std::chrono::day(std::move(a0));
}

std::chrono::day &f13(std::chrono::day &a0, const std::chrono::day &a1) {
  return a0.operator=(a1);
}

std::chrono::day &f14(std::chrono::day &a0, std::chrono::day &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::day f15(const std::chrono::day &a0, const std::chrono::days &a1) {
  return a0 + a1;
}

std::chrono::day f16(const std::chrono::days &a0, const std::chrono::day &a1) {
  return a0 + a1;
}

std::chrono::day f17(const std::chrono::day &a0, const std::chrono::days &a1) {
  return a0 - a1;
}

std::chrono::days f18(const std::chrono::day &a0, const std::chrono::day &a1) {
  return a0 - a1;
}

std::strong_ordering f19(const std::chrono::day &a0,
                         const std::chrono::day &a1) {
  return a0 <=> a1;
}

bool f20(const std::chrono::day &a0, const std::chrono::day &a1) {
  return a0 == a1;
}
