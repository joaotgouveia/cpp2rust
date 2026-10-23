#include <compare>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2>
f33(const std::reverse_iterator<T1> &a0, const std::reverse_iterator<T2> &a1) {
  return a0 <=> a1;
}
