#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
bool f12(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T1> &a1) {
  return a0 != a1;
}
