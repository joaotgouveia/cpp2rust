#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f85(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
using t19 =
    typename std::list<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t20 = typename std::list<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t21 = typename std::list<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t22 = typename std::list<T1, T2>::reverse_iterator::value_type;
