#include <algorithm>
#include <cstddef>
#include <memory_resource>
#include <string>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::pmr::polymorphic_allocator<T1>;

template <typename T1>
using t2 = typename std::pmr::polymorphic_allocator<T1>::value_type;

template <typename T1> std::pmr::polymorphic_allocator<T1> f1() {
  return std::pmr::polymorphic_allocator<T1>();
}

template <typename T1>
std::pmr::polymorphic_allocator<T1> f2(std::pmr::memory_resource *a0) {
  return std::pmr::polymorphic_allocator<T1>(a0);
}

template <typename T1>
std::pmr::polymorphic_allocator<T1>
f3(const std::pmr::polymorphic_allocator<T1> &a0) {
  return std::pmr::polymorphic_allocator<T1>(a0);
}

template <typename T1>
T1 *f4(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1>
void f5(std::pmr::polymorphic_allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1>
std::pmr::polymorphic_allocator<T1>
f6(const std::pmr::polymorphic_allocator<T1> &a0) {
  return a0.select_on_container_copy_construction();
}

template <typename T1>
std::pmr::memory_resource *f7(const std::pmr::polymorphic_allocator<T1> &a0) {
  return a0.resource();
}

template <typename T1, typename T2>
std::pmr::polymorphic_allocator<T1>
f8(const std::pmr::polymorphic_allocator<T2> &a0) {
  return std::pmr::polymorphic_allocator<T1>(a0);
}

template <typename T1, typename T2, typename... T3>
void f9(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return a0.template construct<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
void f10(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return a0.template destroy<T2>(a1);
}

template <typename T1>
bool f11(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T1> &a1) {
  return a0 == a1;
}
