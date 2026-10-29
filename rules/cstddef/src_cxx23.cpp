#include <algorithm>
#include <cstddef>
#include <utility>

using t1 = std::byte;

std::byte f1(std::byte a0, std::byte a1) { return std::operator&(a0, a1); }

std::byte &f2(std::byte &a0, std::byte a1) { return std::operator&=(a0, a1); }

template <typename T1> auto f3(std::byte a0, T1 a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1> auto f4(std::byte &a0, T1 a1) {
  return std::operator<<=(a0, std::move(a1));
}

template <typename T1> auto f5(std::byte a0, T1 a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1> auto f6(std::byte &a0, T1 a1) {
  return std::operator>>=(a0, std::move(a1));
}

std::byte f7(std::byte a0, std::byte a1) { return std::operator^(a0, a1); }

std::byte &f8(std::byte &a0, std::byte a1) { return std::operator^=(a0, a1); }

std::byte f9(std::byte a0, std::byte a1) { return std::operator|(a0, a1); }

std::byte &f10(std::byte &a0, std::byte a1) { return std::operator|=(a0, a1); }

std::byte f11(std::byte a0) { return std::operator~(a0); }
