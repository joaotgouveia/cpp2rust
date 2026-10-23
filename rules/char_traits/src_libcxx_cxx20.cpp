#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

int f182(const char8_t *a0, const char8_t *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f183(const char8_t *a0) {
  return std::char_traits<char8_t>::length(a0);
}
