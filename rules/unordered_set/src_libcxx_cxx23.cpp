#include <initializer_list>
#include <ranges>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f134(std::from_range_t a0, T5 &&a1,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a2, const T2 &a3,
     const T3 &a4, const T4 &a5) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f135(std::from_range_t a0, T5 &&a1,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a2, const T4 &a3) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f136(std::from_range_t a0, T5 &&a1,
     typename std::unordered_set<T1, T2, T3, T4>::size_type a2, const T2 &a3,
     const T4 &a4) {
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            a2, a3, a4);
}
