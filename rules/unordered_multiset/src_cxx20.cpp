#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f68(const std::unordered_multiset<T1, T2, T3, T4> &a0,
         const typename std::unordered_multiset<T1, T2, T3, T4>::key_type &a1) {
  return a0.contains(a1);
}
