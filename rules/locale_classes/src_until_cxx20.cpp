#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <string>
#include <utility>

bool f26(const std::locale &a0, const std::locale &a1) {
  return a0.operator!=(a1);
}
