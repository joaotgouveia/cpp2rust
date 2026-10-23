#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

std::strong_ordering f11(const std::error_category &a0,
                         const std::error_category &a1) {
  return a0.operator<=>(a1);
}
