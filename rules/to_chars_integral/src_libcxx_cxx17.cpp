#include <charconv>
#include <utility>

template <typename T1, typename std::enable_if<std::is_integral<T1>::value,
                                               int>::type T2 = 0>
std::to_chars_result f1(char *a0, char *a1, T1 a2) {
  return std::to_chars<T1>(a0, a1, a2);
}

template <typename T1, typename std::enable_if<std::is_integral<T1>::value,
                                               int>::type T2 = 0>
std::to_chars_result f2(char *a0, char *a1, T1 a2, int a3) {
  return std::to_chars<T1>(a0, a1, a2, a3);
}
