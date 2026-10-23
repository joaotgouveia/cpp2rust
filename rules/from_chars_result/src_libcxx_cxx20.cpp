#include <charconv>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f6(const std::from_chars_result &a0, const std::from_chars_result &a1) {
  return a0 == a1;
}
