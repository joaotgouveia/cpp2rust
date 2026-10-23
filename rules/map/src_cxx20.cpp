#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::size_type f69(std::map<T1, T2, T3, T4> &a0,
                                                 T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::size_type
f70(std::multimap<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f71(const std::map<T1, T2, T3, T4> &a0,
         const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.contains(a1);
}
