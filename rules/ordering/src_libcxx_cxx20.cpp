#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <utility>

using t1 = std::partial_ordering;

std::partial_ordering f1(const std::partial_ordering &a0) {
  return std::partial_ordering(a0);
}

std::partial_ordering f2(std::partial_ordering &&a0) {
  return std::partial_ordering(std::move(a0));
}

std::partial_ordering &f3(std::partial_ordering &a0,
                          const std::partial_ordering &a1) {
  return a0.operator=(a1);
}

std::partial_ordering &f4(std::partial_ordering &a0,
                          std::partial_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f5(std::partial_ordering a0, std::partial_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::partial_ordering &f6() { return std::partial_ordering::less; }

const std::partial_ordering &f7() { return std::partial_ordering::equivalent; }

const std::partial_ordering &f8() { return std::partial_ordering::greater; }

const std::partial_ordering &f9() { return std::partial_ordering::unordered; }

using t2 = std::strong_ordering;

std::partial_ordering f10(const std::strong_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f11(const std::strong_ordering &a0) {
  return static_cast<std::weak_ordering>(a0);
}

std::strong_ordering f12(const std::strong_ordering &a0) {
  return std::strong_ordering(a0);
}

std::strong_ordering f13(std::strong_ordering &&a0) {
  return std::strong_ordering(std::move(a0));
}

std::strong_ordering &f14(std::strong_ordering &a0,
                          const std::strong_ordering &a1) {
  return a0.operator=(a1);
}

std::strong_ordering &f15(std::strong_ordering &a0, std::strong_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f16(std::strong_ordering a0, std::strong_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::strong_ordering &f17() { return std::strong_ordering::less; }

const std::strong_ordering &f18() { return std::strong_ordering::equal; }

const std::strong_ordering &f19() { return std::strong_ordering::equivalent; }

const std::strong_ordering &f20() { return std::strong_ordering::greater; }

using t3 = std::weak_ordering;

std::partial_ordering f21(const std::weak_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f22(const std::weak_ordering &a0) {
  return std::weak_ordering(a0);
}

std::weak_ordering f23(std::weak_ordering &&a0) {
  return std::weak_ordering(std::move(a0));
}

std::weak_ordering &f24(std::weak_ordering &a0, const std::weak_ordering &a1) {
  return a0.operator=(a1);
}

std::weak_ordering &f25(std::weak_ordering &a0, std::weak_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

bool f26(std::weak_ordering a0, std::weak_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::weak_ordering &f27() { return std::weak_ordering::less; }

const std::weak_ordering &f28() { return std::weak_ordering::equivalent; }

const std::weak_ordering &f29() { return std::weak_ordering::greater; }
