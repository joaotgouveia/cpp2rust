#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <string>
#include <utility>

template <typename T1>
void *f18(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1,
          std::size_t a2) {
  return a0.allocate_bytes(a1, a2);
}

template <typename T1>
void f19(std::pmr::polymorphic_allocator<T1> &a0, void *a1, std::size_t a2,
         std::size_t a3) {
  return a0.deallocate_bytes(a1, a2, a3);
}

template <typename T1, typename T2>
T2 *f20(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1) {
  return a0.template allocate_object<T2>(a1);
}

template <typename T1, typename T2>
void f21(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, std::size_t a2) {
  return a0.template deallocate_object<T2>(a1, a2);
}

template <typename T1, typename T2, typename... T3>
T2 *f22(std::pmr::polymorphic_allocator<T1> &a0, T3 &&...a1) {
  return a0.template new_object<T2, T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2>
void f23(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return a0.template delete_object<T2>(a1);
}
