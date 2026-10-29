#include <flat_map>
#include <flat_set>
#include <future>
#include <tuple>
#include <utility>

const std::allocator_arg_t &f1() { return std::allocator_arg; }

using t1 = std::allocator_arg_t;

std::allocator_arg_t f2() { return std::allocator_arg_t(); }

std::allocator_arg_t f3(const std::allocator_arg_t &a0) {
  return std::allocator_arg_t(a0);
}

std::allocator_arg_t f4(std::allocator_arg_t &&a0) {
  return std::allocator_arg_t(std::move(a0));
}

std::allocator_arg_t &f5(std::allocator_arg_t &a0,
                         const std::allocator_arg_t &a1) {
  return a0.operator=(a1);
}

std::allocator_arg_t &f6(std::allocator_arg_t &a0, std::allocator_arg_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
using t2 = std::uses_allocator<std::promise<T1>, T2>;

template <typename T1, typename T2>
bool f7(const std::uses_allocator<std::promise<T1>, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f8(const std::uses_allocator<std::promise<T1>, T2> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2> bool f9() {
  return std::uses_allocator<std::promise<T1>, T2>::value;
}

template <typename T1, typename T2, typename T3, typename T4>
using t3 = std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4>;

template <typename T1, typename T2, typename T3, typename T4>
bool f10(const std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f11(const std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4> bool f12() {
  return std::uses_allocator<std::flat_multiset<T1, T2, T3>, T4>::value;
}

template <typename T1, typename T2, typename T3, typename T4>
using t4 = std::uses_allocator<std::flat_set<T1, T2, T3>, T4>;

template <typename T1, typename T2, typename T3, typename T4>
bool f13(const std::uses_allocator<std::flat_set<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::uses_allocator<std::flat_set<T1, T2, T3>, T4> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4> bool f15() {
  return std::uses_allocator<std::flat_set<T1, T2, T3>, T4>::value;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
using t5 = std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f16(const std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f17(const std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f18() {
  return std::uses_allocator<std::flat_map<T1, T2, T3, T4, T5>, T6>::value;
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
using t6 = std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f19(
    const std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f20(
    const std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
bool f21() {
  return std::uses_allocator<std::flat_multimap<T1, T2, T3, T4, T5>, T6>::value;
}

template <typename T1, typename T2> using t7 = std::uses_allocator<T1, T2>;

template <typename T1, typename T2> bool f22() {
  return std::uses_allocator_v<T1, T2>;
}
