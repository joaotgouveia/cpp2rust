#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::map<T1, T2, T3, T4>::node_type;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::map<T1, T2, T3, T4>::insert_return_type;

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type
f61(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::node_type f62(std::map<T1, T2, T3, T4> &a0,
                                                 const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::insert_return_type
f63(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::map<T1, T2, T3, T4>::iterator
f64(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1,
    typename std::map<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f65(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f66(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f67(std::map<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f68(std::map<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f69(std::map<T1, T2, T3, T4> &a0, const T1 &a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(a1, std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f70(std::map<T1, T2, T3, T4> &a0, T1 &&a1, T5 &&...a2) {
  return a0.template try_emplace<T5...>(std::move(a1), std::forward<T5>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f71(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, const T1 &a2,
    T5 &&...a3) {
  return a0.template try_emplace<T5...>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1), a2,
      std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
typename std::map<T1, T2, T3, T4>::iterator
f72(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T1 &&a2, T5 &&...a3) {
  return a0.template try_emplace<T5...>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2), std::forward<T5>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f73(std::map<T1, T2, T3, T4> &a0, const T1 &a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f74(std::map<T1, T2, T3, T4> &a0, T1 &&a1, T5 &&a2) {
  return a0.template insert_or_assign<T5>(std::move(a1), std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f75(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, const T1 &a2,
    T5 &&a3) {
  return a0.template insert_or_assign<T5>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1), a2,
      std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::iterator
f76(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T1 &&a2, T5 &&a3) {
  return a0.template insert_or_assign<T5>(
      std::forward<typename std::map<T1, T2, T3, T4>::const_iterator>(a1),
      std::move(a2), std::forward<T5>(a3));
}

template <typename T1, typename T2, typename T3>
using t20 = std::pmr::map<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t21 = std::pmr::multimap<T1, T2, T3>;
