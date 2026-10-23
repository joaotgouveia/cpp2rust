#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1>
  requires(std::is_object_v<T1>)
using t1 = std::incrementable_traits<T1 *>;

template <typename T1>
using t2 = typename std::incrementable_traits<T1 *>::difference_type;

template <typename T1> using t3 = std::incrementable_traits<const T1>;

template <typename T1> using t4 = std::incrementable_traits<T1>;

template <typename T1>
using t5 = typename std::incrementable_traits<T1>::difference_type;

template <typename T1, typename T2>
using t6 = std::incrementable_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t7 = typename std::incrementable_traits<
    std::common_iterator<T1, T2>>::difference_type;
