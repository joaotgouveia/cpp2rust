#include <algorithm>
#include <compare>
#include <cstddef>
#include <cxxabi.h>
#include <exception>
#include <typeindex>
#include <utility>

std::strong_ordering f14(const std::type_index &a0, const std::type_index &a1) {
  return a0.operator<=>(a1);
}
