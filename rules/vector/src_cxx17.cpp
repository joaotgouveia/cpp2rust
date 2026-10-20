#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

template <typename T1> using t54 = std::pmr::vector<T1>;

template <typename T1, typename... T2>
typename std::vector<bool, T1>::reference f185(std::vector<bool, T1> &a0,
                                               T2 &&...a1) {
  return a0.template emplace_back<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::vector<T1, T2>::reference f186(std::vector<T1, T2> &a0,
                                             T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
