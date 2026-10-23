#include <charconv>
#include <utility>

bool f6(const std::from_chars_result &a0, const std::from_chars_result &a1) {
  return operator==(a0, a1);
}
