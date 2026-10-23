#include <cstddef>
#include <memory_resource>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
bool f24(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T2> &a1) {
  return std::pmr::operator!= <T1, T2>(a0, a1);
}
