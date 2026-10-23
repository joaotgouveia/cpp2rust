#include <algorithm>
#include <barrier>
#include <compare>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer>
f41(const std::unique_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::compare_three_way_result_t<typename std::unique_ptr<T1, T2>::pointer,
                                typename std::unique_ptr<T3, T4>::pointer>
f42(const std::unique_ptr<T1, T2> &a0, const std::unique_ptr<T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}
