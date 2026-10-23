#include <charconv>
#include <utility>

std::to_chars_result f1(char *a0, char *a1, long double a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f2(char *a0, char *a1, long double a2,
                        std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f3(char *a0, char *a1, long double a2,
                        std::chars_format a3, int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}

std::to_chars_result f4(char *a0, char *a1, double a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f5(char *a0, char *a1, double a2, std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f6(char *a0, char *a1, double a2, std::chars_format a3,
                        int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}

std::to_chars_result f7(char *a0, char *a1, float a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f8(char *a0, char *a1, float a2, std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f9(char *a0, char *a1, float a2, std::chars_format a3,
                        int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}
