#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f72(std::from_range_t a0, T4 &&a1, const T2 &a2,
                              const T3 &a3) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f73(std::from_range_t a0, T4 &&a1, const T3 &a2) {
  return std::multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}
