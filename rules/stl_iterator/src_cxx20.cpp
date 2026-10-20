#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
std::compare_three_way_result_t<T1> f34(const std::move_iterator<T1> &a0,
                                        const std::move_iterator<T1> &a1) {
  return a0 <=> a1;
}

template <typename T1>
std::compare_three_way_result_t<T1, T1>
f35(const std::reverse_iterator<T1> &a0, const std::reverse_iterator<T1> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f36(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2>
f37(const std::reverse_iterator<T1> &a0, const std::reverse_iterator<T2> &a1) {
  return a0 <=> a1;
}
