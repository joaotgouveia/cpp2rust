#include <algorithm>
#include <string>
#include <utility>

template <typename T1, typename T2, typename... T3>
T1 f1(const T2 &a0, T3 &&...a1) {
  return std::make_obj_using_allocator<T1, T2, T3...>(a0,
                                                      std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 *f2(T1 *a0, const T2 &a1, T3 &&...a2) {
  return std::uninitialized_construct_using_allocator<T1, T2, T3...>(
      a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2> auto f3(const T2 &a0) {
  return std::uses_allocator_construction_args<T1, T2>(a0);
}

template <typename T1, typename T2, typename... T3>
auto f4(const T2 &a0, T3 &&...a1) {
  return std::uses_allocator_construction_args<T1, T2, T3...>(
      a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f5(const T2 &a0, std::piecewise_construct_t a1, T3 &&a2, T4 &&a3) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(
      a0, std::move(a1), std::forward<T3>(a2), std::forward<T4>(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f6(const T2 &a0, const std::pair<T3, T4> &&a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0,
                                                               std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f7(const T2 &a0, std::pair<T3, T4> &&a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0,
                                                               std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f8(const T2 &a0, T3 &&a1, T4 &&a2) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(
      a0, std::forward<T3>(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f9(const T2 &a0, const std::pair<T3, T4> &a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f10(const T2 &a0, std::pair<T3, T4> &a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0, a1);
}
