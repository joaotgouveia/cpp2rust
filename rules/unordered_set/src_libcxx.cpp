#include <initializer_list>
#include <unordered_set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f93(typename std::unordered_set<T1, T2, T3, T4>::size_type a0,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a1,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a2) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f94(std::initializer_list<
    typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0) {
  return std::unordered_set<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unordered_set<T1, T2, T3, T4>
f95(std::initializer_list<
        typename std::unordered_set<T1, T2, T3, T4>::value_type>
        a0,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a1,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a2,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a3) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4> f96(T5 a0, T5 a1) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f97(T5 a0, T5 a1, typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a4) {
  return std::unordered_set<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}
