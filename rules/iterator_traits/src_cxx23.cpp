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

template <typename T1> using t17 = std::iterator_traits<T1>;
