#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f49(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_set<T1, T2, T3, T4>::size_type
f50(std::unordered_set<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, std::move(a1));
}
