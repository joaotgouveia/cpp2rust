#include <compare>
#include <string.h>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::strong_ordering f19(const std::error_condition &a0,
                         const std::error_condition &a1) {
  return a0 <=> a1;
}
