#include <compare>
#include <iosfwd>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::strong_ordering f57(const std::error_category &a0,
                         const std::error_category &a1) {
  return a0.operator<=>(a1);
}

std::strong_ordering f58(const std::error_code &a0, const std::error_code &a1) {
  return a0 <=> a1;
}

std::strong_ordering f59(const std::error_condition &a0,
                         const std::error_condition &a1) {
  return a0 <=> a1;
}
