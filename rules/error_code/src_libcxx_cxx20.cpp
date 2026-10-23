#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::strong_ordering f24(const std::error_code &a0, const std::error_code &a1) {
  return a0 <=> a1;
}
