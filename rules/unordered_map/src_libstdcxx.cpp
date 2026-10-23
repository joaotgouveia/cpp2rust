#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t18 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t19 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t20 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t21 = typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t22 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t23 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t24 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t25 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t26 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t27 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t28 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t29 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t30 =
    typename std::unordered_map<T1, T2, T3, T4, T5>::local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t31 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t32 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::local_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t33 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t34 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t35 = typename std::unordered_map<T1, T2, T3, T4,
                                        T5>::const_local_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t36 =
    typename std::unordered_map<T1, T2, T3, T4,
                                T5>::const_local_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t37 =
    typename std::unordered_map<T1, T2, T3, T4,
                                T5>::const_local_iterator::iterator_category;
