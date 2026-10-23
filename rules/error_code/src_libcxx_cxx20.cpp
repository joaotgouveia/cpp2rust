#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

std::strong_ordering f24(const std::error_code &a0, const std::error_code &a1) {
  return std::operator<=>(a0, a1);
}
