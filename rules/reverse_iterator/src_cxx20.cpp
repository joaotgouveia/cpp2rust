#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
using t6 = typename std::reverse_iterator<T1>::iterator_concept;

template <typename T1>
using t7 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t8 = typename std::reverse_iterator<T1>::value_type;

template <typename T1>
std::iter_rvalue_reference_t<T1> f19(const std::reverse_iterator<T1> &a0) {
  return iter_move(a0);
}
