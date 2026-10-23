#include <algorithm>
#include <cstddef>
#include <cxxabi.h>
#include <exception>
#include <typeindex>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f13(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator!=(a1);
}
