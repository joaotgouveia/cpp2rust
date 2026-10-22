#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <stack>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T2> f43(const std::stack<T1, T2> &a0,
                                        const std::stack<T1, T2> &a1) {
  return a0 <=> a1;
}
