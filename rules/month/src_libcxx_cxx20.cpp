#include <chrono>
#include <compare>
#include <utility>

using t1 = std::chrono::month;

std::chrono::month f1() { return std::chrono::month(); }

std::chrono::month f2(unsigned int a0) { return std::chrono::month(a0); }

std::chrono::month &f3(std::chrono::month &a0) { return a0.operator++(); }

std::chrono::month f4(std::chrono::month &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::month &f5(std::chrono::month &a0) { return a0.operator--(); }

std::chrono::month f6(std::chrono::month &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::month &f7(std::chrono::month &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::month &f8(std::chrono::month &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

unsigned int f9(const std::chrono::month &a0) {
  return static_cast<unsigned int>(a0);
}

bool f10(const std::chrono::month &a0) { return a0.ok(); }

std::chrono::month f11(const std::chrono::month &a0) {
  return std::chrono::month(a0);
}

std::chrono::month f12(std::chrono::month &&a0) {
  return std::chrono::month(std::move(a0));
}

std::chrono::month &f13(std::chrono::month &a0, const std::chrono::month &a1) {
  return a0.operator=(a1);
}

std::chrono::month &f14(std::chrono::month &a0, std::chrono::month &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month f15(const std::chrono::months &a0,
                       const std::chrono::month &a1) {
  return a0 + a1;
}

std::chrono::month f16(const std::chrono::month &a0,
                       const std::chrono::months &a1) {
  return a0 + a1;
}

std::chrono::month f17(const std::chrono::month &a0,
                       const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::months f18(const std::chrono::month &a0,
                        const std::chrono::month &a1) {
  return a0 - a1;
}

std::strong_ordering f19(const std::chrono::month &a0,
                         const std::chrono::month &a1) {
  return a0 <=> a1;
}

bool f20(const std::chrono::month &a0, const std::chrono::month &a1) {
  return a0 == a1;
}
