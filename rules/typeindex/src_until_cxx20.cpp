#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <exception>
#include <typeindex>
#include <utility>

bool f13(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator!=(a1);
}
