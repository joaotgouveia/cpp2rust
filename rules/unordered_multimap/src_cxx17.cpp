#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type
f65(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type
f66(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f67(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f68(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}
