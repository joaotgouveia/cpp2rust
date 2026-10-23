#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f10(const std::pmr::memory_resource &a0,
         const std::pmr::memory_resource &a1) {
  return a0 != a1;
}
