#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f65(std::list<T1, T2> &a0, const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f66(std::list<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f67(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f68(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f69(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2>
using t17 =
    typename std::list<T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t18 = typename std::list<T1, T2>::reverse_iterator::iterator_concept;
