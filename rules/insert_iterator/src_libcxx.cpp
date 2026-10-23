#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t4 = typename std::insert_iterator<T1>::iterator_category;

template <typename T1> using t5 = typename std::insert_iterator<T1>::value_type;

template <typename T1>
using t6 = typename std::insert_iterator<T1>::difference_type;

template <typename T1> using t7 = typename std::insert_iterator<T1>::pointer;

template <typename T1> using t8 = typename std::insert_iterator<T1>::reference;
