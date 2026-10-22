#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f76(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f77(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f78(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f79(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}
