#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f81(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::unordered_set<T1, T2, T3, T4>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t16 =
    typename std::unordered_set<T1, T2, T3, T4>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t17 =
    typename std::unordered_set<T1, T2, T3, T4>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t18 = typename std::unordered_set<T1, T2, T3, T4>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = typename std::unordered_set<T1, T2, T3, T4>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t20 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t21 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t22 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t23 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t24 =
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t25 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t26 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t27 =
    typename std::unordered_set<T1, T2, T3, T4>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t28 = typename std::unordered_set<T1, T2, T3,
                                        T4>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::unordered_set<T1, T2, T3,
                                        T4>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t30 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t31 =
    typename std::unordered_set<T1, T2, T3, T4>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t32 = typename std::unordered_set<T1, T2, T3,
                                        T4>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t33 =
    typename std::unordered_set<T1, T2, T3,
                                T4>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t34 =
    typename std::unordered_set<T1, T2, T3,
                                T4>::const_local_iterator::iterator_category;
