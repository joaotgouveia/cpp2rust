#include <algorithm>
#include <compare>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
using t6 = typename std::reverse_iterator<T1>::iterator_concept;

template <typename T1>
std::iter_rvalue_reference_t<T1> f13(const std::reverse_iterator<T1> &a0) {
  return iter_move(a0);
}
