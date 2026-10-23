#include <algorithm>
#include <initializer_list>
#include <ranges>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f54(std::from_range_t a0, T4 &&a1, const T2 &a2,
                         const T3 &a3) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f55(std::from_range_t a0, T4 &&a1, const T3 &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f56(std::set<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}
