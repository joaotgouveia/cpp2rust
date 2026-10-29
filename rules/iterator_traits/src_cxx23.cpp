#include <algorithm>
#include <utility>

template <typename T1>
  requires(std::is_object_v<T1>)
using t1 = std::iterator_traits<T1 *>;

template <typename T1>
using t2 = typename std::iterator_traits<T1 *>::iterator_concept;

template <typename T1>
using t3 = typename std::iterator_traits<T1 *>::iterator_category;

template <typename T1>
using t4 = typename std::iterator_traits<T1 *>::value_type;

template <typename T1>
using t5 = typename std::iterator_traits<T1 *>::difference_type;

template <typename T1> using t6 = typename std::iterator_traits<T1 *>::pointer;

template <typename T1>
using t7 = typename std::iterator_traits<T1 *>::reference;

template <typename T1>
using t8 = std::iterator_traits<std::counted_iterator<T1>>;

template <typename T1>
using t9 = typename std::iterator_traits<std::counted_iterator<T1>>::pointer;

template <typename T1, typename T2>
using t10 = std::iterator_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t11 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_concept;

template <typename T1, typename T2>
using t12 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::iterator_category;

template <typename T1, typename T2>
using t13 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::value_type;

template <typename T1, typename T2>
using t14 = typename std::iterator_traits<
    std::common_iterator<T1, T2>>::difference_type;

template <typename T1, typename T2>
using t15 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::pointer;

template <typename T1, typename T2>
using t16 =
    typename std::iterator_traits<std::common_iterator<T1, T2>>::reference;

template <typename T1> using t17 = std::iterator_traits<T1>;
