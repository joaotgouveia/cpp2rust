#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t11 = typename std::istream_iterator<T1, T2, T3, T4>::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t12 = typename std::istream_iterator<T1, T2, T3, T4>::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t13 = typename std::istream_iterator<T1, T2, T3, T4>::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t14 = typename std::istream_iterator<T1, T2, T3, T4>::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t15 = typename std::istream_iterator<T1, T2, T3, T4>::reference;
