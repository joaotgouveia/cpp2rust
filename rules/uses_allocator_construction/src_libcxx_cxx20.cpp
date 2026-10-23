#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3>
auto f1(const T2 &a0, T3 &&...a1) {
  return std::make_obj_using_allocator<T1, T2, T3...>(a0,
                                                      std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
auto f2(T1 *a0, const T2 &a1, T3 &&...a2) {
  return std::uninitialized_construct_using_allocator<T1, T2, T3...>(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename... T3>
auto f3(const T2 &a0, T3 &&...a1) {
  return std::uses_allocator_construction_args<T1, T2, T3...>(
      a0, std::forward<T3>(a1)...);
}
