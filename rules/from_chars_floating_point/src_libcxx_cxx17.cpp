#include <charconv>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::from_chars_result f1(const char *a0, const char *a1, double &a2,
                          std::chars_format a3) {
  return std::from_chars(a0, a1, a2, a3);
}

std::from_chars_result f2(const char *a0, const char *a1, float &a2,
                          std::chars_format a3) {
  return std::from_chars(a0, a1, a2, a3);
}
