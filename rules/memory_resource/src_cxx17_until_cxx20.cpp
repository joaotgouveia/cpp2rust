#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <utility>

bool f10(const std::pmr::memory_resource &a0,
         const std::pmr::memory_resource &a1) {
  return std::pmr::operator!=(a0, a1);
}
