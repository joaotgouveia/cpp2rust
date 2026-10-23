#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f19(const std::istreambuf_iterator<T1, T2> &a0,
         const std::istreambuf_iterator<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
using t13 = typename std::istreambuf_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t14 = typename std::istreambuf_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t15 = typename std::istreambuf_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t16 = typename std::istreambuf_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t17 = typename std::istreambuf_iterator<T1, T2>::reference;
