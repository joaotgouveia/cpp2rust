#include <charconv>
#include <utility>

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::from_chars_result f1(const char *a0, const char *a1, T1 &a2) {
  return std::from_chars<T1>(a0, a1, a2);
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::from_chars_result f2(const char *a0, const char *a1, T1 &a2, int a3) {
  return std::from_chars<T1>(a0, a1, a2, a3);
}
