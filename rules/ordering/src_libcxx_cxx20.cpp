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

template <typename T1>
std::partial_ordering f5(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1>
std::partial_ordering f6(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f7(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f8(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f9(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f10(T1 &&a0, std::partial_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f11(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f12(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f13(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f14(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

template <typename T1> bool f15(std::partial_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

bool f16(std::partial_ordering a0, std::partial_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::partial_ordering &f17() { return std::partial_ordering::less; }

const std::partial_ordering &f18() { return std::partial_ordering::equivalent; }

const std::partial_ordering &f19() { return std::partial_ordering::greater; }

const std::partial_ordering &f20() { return std::partial_ordering::unordered; }

using t2 = std::strong_ordering;

std::partial_ordering f21(const std::strong_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f22(const std::strong_ordering &a0) {
  return static_cast<std::weak_ordering>(a0);
}

std::strong_ordering f23(const std::strong_ordering &a0) {
  return std::strong_ordering(a0);
}

std::strong_ordering f24(std::strong_ordering &&a0) {
  return std::strong_ordering(std::move(a0));
}

std::strong_ordering &f25(std::strong_ordering &a0,
                          const std::strong_ordering &a1) {
  return a0.operator=(a1);
}

std::strong_ordering &f26(std::strong_ordering &a0, std::strong_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::strong_ordering f27(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1>
std::strong_ordering f28(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f29(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f30(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f31(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f32(T1 &&a0, std::strong_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f33(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f34(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f35(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f36(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

template <typename T1> bool f37(std::strong_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

bool f38(std::strong_ordering a0, std::strong_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::strong_ordering &f39() { return std::strong_ordering::less; }

const std::strong_ordering &f40() { return std::strong_ordering::equal; }

const std::strong_ordering &f41() { return std::strong_ordering::equivalent; }

const std::strong_ordering &f42() { return std::strong_ordering::greater; }

using t3 = std::weak_ordering;

std::partial_ordering f43(const std::weak_ordering &a0) {
  return static_cast<std::partial_ordering>(a0);
}

std::weak_ordering f44(const std::weak_ordering &a0) {
  return std::weak_ordering(a0);
}

std::weak_ordering f45(std::weak_ordering &&a0) {
  return std::weak_ordering(std::move(a0));
}

std::weak_ordering &f46(std::weak_ordering &a0, const std::weak_ordering &a1) {
  return a0.operator=(a1);
}

std::weak_ordering &f47(std::weak_ordering &a0, std::weak_ordering &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::weak_ordering f48(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) <=> std::move(a1);
}

template <typename T1> std::weak_ordering f49(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) <=> std::forward<T1>(a1);
}

template <typename T1> bool f50(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) >= std::move(a1);
}

template <typename T1> bool f51(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) > std::move(a1);
}

template <typename T1> bool f52(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) <= std::move(a1);
}

template <typename T1> bool f53(T1 &&a0, std::weak_ordering a1) {
  return std::forward<T1>(a0) < std::move(a1);
}

template <typename T1> bool f54(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) >= std::forward<T1>(a1);
}

template <typename T1> bool f55(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) > std::forward<T1>(a1);
}

template <typename T1> bool f56(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) <= std::forward<T1>(a1);
}

template <typename T1> bool f57(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) < std::forward<T1>(a1);
}

template <typename T1> bool f58(std::weak_ordering a0, T1 &&a1) {
  return std::move(a0) == std::forward<T1>(a1);
}

bool f59(std::weak_ordering a0, std::weak_ordering a1) {
  return std::move(a0) == std::move(a1);
}

const std::weak_ordering &f60() { return std::weak_ordering::less; }

const std::weak_ordering &f61() { return std::weak_ordering::equivalent; }

const std::weak_ordering &f62() { return std::weak_ordering::greater; }
