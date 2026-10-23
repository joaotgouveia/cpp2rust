#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type
f57(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type
f58(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::insert_return_type
f59(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f60(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f61(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f62(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::key_type &&a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(std::move(a1), std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f63(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    const typename std::map<T1, T2, T3, T4>::key_type &a2, T5 &&...a3) {
  return a0.template try_emplace<T5...>(a1, a2, std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f64(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::key_type &&a2, T5 &&...a3) {
  return a0.template try_emplace<T5...>(a1, std::move(a2),
                                        std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f65(std::map<T1, T2, T3, T4> &a0,
    const typename std::map<T1, T2, T3, T4>::key_type &a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f66(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::key_type &&a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(std::move(a1), std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f67(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    const typename std::map<T1, T2, T3, T4>::key_type &a2, T5 &&a3) {
  return a0.template insert_or_assign<T5>(a1, a2, std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f68(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::key_type &&a2, T5 &&a3) {
  return a0.template insert_or_assign<T5>(a1, std::move(a2),
                                          std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3>
using t14 = std::pmr::map<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t15 = std::pmr::multimap<T1, T2, T3>;
