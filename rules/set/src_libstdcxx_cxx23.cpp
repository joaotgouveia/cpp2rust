#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f74(std::from_range_t a0, T4 &&a1, const T2 &a2,
                         const T3 &a3) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f75(std::from_range_t a0, T4 &&a1, const T3 &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}
