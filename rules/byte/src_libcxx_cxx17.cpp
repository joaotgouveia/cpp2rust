#include <algorithm>
#include <cstddef>
#include <utility>

using t1 = std::byte;

std::byte f1(std::byte a0, std::byte a1) { return std::operator&(a0, a1); }

std::byte &f2(std::byte &a0, std::byte a1) { return std::operator&=(a0, a1); }

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::byte f3(std::byte a0, T1 a1) {
  return std::operator<< <T1>(a0, std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::byte &f4(std::byte &a0, T1 a1) {
  return std::operator<<= <T1>(a0, std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::byte f5(std::byte a0, T1 a1) {
  return std::operator>> <T1>(a0, std::move(a1));
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::byte &f6(std::byte &a0, T1 a1) {
  return std::operator>>= <T1>(a0, std::move(a1));
}

std::byte f7(std::byte a0, std::byte a1) { return std::operator^(a0, a1); }

std::byte &f8(std::byte &a0, std::byte a1) { return std::operator^=(a0, a1); }

std::byte f9(std::byte a0, std::byte a1) { return std::operator|(a0, a1); }

std::byte &f10(std::byte &a0, std::byte a1) { return std::operator|=(a0, a1); }

std::byte f11(std::byte a0) { return std::operator~(a0); }

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
T1 f12(std::byte a0) {
  return std::to_integer<T1>(a0);
}
