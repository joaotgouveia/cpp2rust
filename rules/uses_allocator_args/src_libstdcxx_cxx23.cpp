#include <algorithm>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f9(const T2 &a0, const std::pair<T3, T4> &&a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0,
                                                               std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
auto f10(const T2 &a0, std::pair<T3, T4> &a1) {
  return std::uses_allocator_construction_args<T1, T2, T3, T4>(a0, a1);
}
