#include <algorithm>
#include <initializer_list>
#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::vector<bool, T1> f195(std::from_range_t a0, T2 &&a1, const T1 &a2) {
  return std::vector<bool, T1>(std::move(a0), std::forward<T2>(a1), a2);
}

template <typename T1>
const typename std::vector<bool, T1>::reference &
f196(const typename std::vector<bool, T1>::reference &a0, bool a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::vector<T1, T2> f197(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::vector<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
