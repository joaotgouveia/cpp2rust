#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1>
using t7 = typename std::move_iterator<T1>::iterator_category;
