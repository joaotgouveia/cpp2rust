#include <ios>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f26(const std::locale &a0, const std::locale &a1) {
  return a0.operator!=(a1);
}
