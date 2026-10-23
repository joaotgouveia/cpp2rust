#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_multiset<T1, T2, T3, T4>::node_type
f73(std::unordered_multiset<T1, T2, T3, T4> &a0,
    const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.extract(a1);
}
