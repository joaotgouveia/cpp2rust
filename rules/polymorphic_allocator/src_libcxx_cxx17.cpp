#include <cstddef>
#include <memory_resource>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
bool f26(const std::pmr::polymorphic_allocator<T1> &a0,
         const std::pmr::polymorphic_allocator<T2> &a1) {
  return std::pmr::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename... T4, typename... T5>
void f27(std::pmr::polymorphic_allocator<T1> &a0, std::pair<T2, T3> *a1,
         std::piecewise_construct_t a2, std::tuple<T4...> a3,
         std::tuple<T5...> a4) {
  return a0.template construct<T2, T3, T4...>(
      a1, std::move(a2), std::forward<std::tuple<T4...>>(a3),
      std::forward<std::tuple<T5...>>(a4));
}

template <typename T1, typename T2, typename T3>
void f28(std::pmr::polymorphic_allocator<T1> &a0, std::pair<T2, T3> *a1) {
  return a0.template construct<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f29(std::pmr::polymorphic_allocator<T1> &a0, std::pair<T2, T3> *a1,
         T4 &&a2, T5 &&a3) {
  return a0.template construct<T2, T3, T4, T5>(a1, std::forward<T4>(a2),
                                               std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f30(std::pmr::polymorphic_allocator<T1> &a0, std::pair<T2, T3> *a1,
         const std::pair<T4, T5> &a2) {
  return a0.template construct<T2, T3, T4, T5>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f31(std::pmr::polymorphic_allocator<T1> &a0, std::pair<T2, T3> *a1,
         std::pair<T4, T5> &&a2) {
  return a0.template construct<T2, T3, T4, T5>(a1, std::move(a2));
}
