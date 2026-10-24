#include <flat_map>
#include <flat_set>
#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t4 = std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4>;

template <typename T1, typename T2, typename T3, typename T4>
bool f5(const std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f6(const std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4>
using t5 = std::uses_allocator<std::flat_set<T1, T2, T3>, T4>;

template <typename T1, typename T2, typename T3, typename T4>
bool f7(const std::uses_allocator<std::flat_set<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f8(const std::uses_allocator<std::flat_set<T1, T2, T3>, T4> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
using t6 = std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f9(const std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f10(const std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
using t7 = std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f11(
    const std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f12(
    const std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6> &a0) {
  return a0.operator()();
}
