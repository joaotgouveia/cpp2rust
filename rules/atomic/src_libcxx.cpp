#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> std::atomic<T1 *> f366() { return std::atomic<T1 *>(); }

template <typename T1> std::atomic<T1 *> f367(T1 *a0) {
  return std::atomic<T1 *>(a0);
}

template <typename T1> std::atomic<T1> f368() { return std::atomic<T1>(); }

template <typename T1> std::atomic<T1> f369(T1 a0) {
  return std::atomic<T1>(a0);
}

template <typename T1>
auto f370(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1>
bool f371(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f372(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f373(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f374(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f375(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f376(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f377(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f378(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
T1 f379(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f380(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f381(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f382(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f383(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1>
T1 f384(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1>
T1 f385(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f386(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f387(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1>
T1 f388(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1>
T1 f389(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f390(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f391(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1>
T1 f392(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1>
T1 f393(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f394(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f395(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f396(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f397(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f398(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}
