#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f91(std::from_range_t a0, T5 &&a1, const T3 &a2,
                             const T4 &a3) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f92(std::from_range_t a0, T5 &&a1, const T4 &a2) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}
