#include <algorithm>
#include <string>
#include <utility>

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
void *f6(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1,
         std::size_t a2) {
  return a0.allocate_bytes(a1, a2);
}

template <typename T1>
void f7(std::pmr::polymorphic_allocator<T1> &a0, void *a1, std::size_t a2,
        std::size_t a3) {
  return a0.deallocate_bytes(a1, a2, a3);
}

template <typename T1>
std::pmr::polymorphic_allocator<T1>
f8(const std::pmr::polymorphic_allocator<T1> &a0) {
  return a0.select_on_container_copy_construction();
}

template <typename T1>
std::pmr::memory_resource *f9(const std::pmr::polymorphic_allocator<T1> &a0) {
  return a0.resource();
}

template <typename T1, typename T2>
std::pmr::polymorphic_allocator<T1>
f10(const std::pmr::polymorphic_allocator<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::pmr::polymorphic_allocator<T1>(a0);
}

template <typename T1, typename T2>
T2 *f11(std::pmr::polymorphic_allocator<T1> &a0, std::size_t a1) {
  return a0.template allocate_object<T2>(a1);
}

template <typename T1, typename T2>
void f12(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, std::size_t a2) {
  return a0.template deallocate_object<T2>(a1, a2);
}

template <typename T1, typename T2, typename... T3>
T2 *f13(std::pmr::polymorphic_allocator<T1> &a0, T3 &&...a1) {
  return a0.template new_object<T2, T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2>
void f14(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return a0.template delete_object<T2>(a1);
}

template <typename T1, typename T2, typename... T3>
void f15(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1, T3 &&...a2) {
  return a0.template construct<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
void f16(std::pmr::polymorphic_allocator<T1> &a0, T2 *a1) {
  return a0.template destroy<T2>(a1);
}

template <typename T1>
bool f17(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T1> &a1) {
  return operator==(a0, a1);
}
