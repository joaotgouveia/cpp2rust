#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

bool f154(const std::locale &a0, const std::locale &a1) {
  return a0.operator!=(a1);
}
