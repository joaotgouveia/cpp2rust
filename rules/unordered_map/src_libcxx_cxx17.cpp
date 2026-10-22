#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f112(
    std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_map<
        typename std::unordered_map<T1, T2, T3, T4, T5>::key_type,
        typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type, T6, T7,
        typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f113(
    std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_map<
        typename std::unordered_map<T1, T2, T3, T4, T5>::key_type,
        typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type, T6, T7,
        typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f114(
    std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_multimap<
        typename std::unordered_map<T1, T2, T3, T4, T5>::key_type,
        typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type, T6, T7,
        typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f115(
    std::unordered_map<T1, T2, T3, T4, T5> &a0,
    std::unordered_multimap<
        typename std::unordered_map<T1, T2, T3, T4, T5>::key_type,
        typename std::unordered_map<T1, T2, T3, T4, T5>::mapped_type, T6, T7,
        typename std::unordered_map<T1, T2, T3, T4, T5>::allocator_type> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}
