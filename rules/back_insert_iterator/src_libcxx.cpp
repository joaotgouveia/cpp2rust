#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1>
using t4 = typename std::back_insert_iterator<T1>::iterator_category;

template <typename T1>
using t5 = typename std::back_insert_iterator<T1>::value_type;

template <typename T1>
using t6 = typename std::back_insert_iterator<T1>::difference_type;

template <typename T1>
using t7 = typename std::back_insert_iterator<T1>::pointer;

template <typename T1>
using t8 = typename std::back_insert_iterator<T1>::reference;

template <typename T1> std::back_insert_iterator<T1> f7(T1 &a0) {
  return std::back_inserter<T1>(a0);
}
