#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <exception>
#include <typeinfo>
#include <utility>

bool f13(const std::type_info &a0, const std::type_info &a1) {
  return a0.operator!=(a1);
}
