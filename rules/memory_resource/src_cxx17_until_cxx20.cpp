#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <memory_resource>
#include <string>
#include <utility>

bool f44(const std::pmr::memory_resource &a0,
         const std::pmr::memory_resource &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f45(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T2> &a1) {
  return a0 != a1;
}
