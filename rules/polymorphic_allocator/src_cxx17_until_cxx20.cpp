#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <tuple>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::pmr::polymorphic_allocator<T1>
f11(const std::pmr::polymorphic_allocator<T2> &a0) {
  return std::pmr::polymorphic_allocator<T1>(a0);
}

template <typename T1>
bool f12(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T1> &a1) {
  return operator!=(a0, a1);
}
