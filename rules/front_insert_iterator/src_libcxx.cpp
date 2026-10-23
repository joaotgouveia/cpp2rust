#include <iterator>
#include <utility>

template <typename T1>
using t4 = typename std::front_insert_iterator<T1>::iterator_category;

template <typename T1>
using t5 = typename std::front_insert_iterator<T1>::value_type;

template <typename T1>
using t6 = typename std::front_insert_iterator<T1>::difference_type;

template <typename T1>
using t7 = typename std::front_insert_iterator<T1>::pointer;

template <typename T1>
using t8 = typename std::front_insert_iterator<T1>::reference;

template <typename T1> std::front_insert_iterator<T1> f7(T1 &a0) {
  return std::front_inserter<T1>(a0);
}
