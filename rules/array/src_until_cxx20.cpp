#include <array>
#include <utility>

template <typename T1, std::size_t T2>
bool f33(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f34(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f35(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f36(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f37(const std::array<T1, T2> &a0, const std::array<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
