#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

int f122(const char8_t *a0, const char8_t *a1, std::size_t a2) {
  return std::char_traits<char8_t>::compare(a0, a1, a2);
}

std::size_t f123(const char8_t *a0) {
  return std::char_traits<char8_t>::length(a0);
}
