#include "../common/init.h"
#include "../common/traits.h"
#include <string.h>
#include <system_error>
#include <utility>

bool f9(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator!=(a1);
}

bool f10(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator<(a1);
}
