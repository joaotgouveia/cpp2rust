#include <compare>
#include <iterator>
#include <utility>

template <typename T1> auto f39(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f40(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f41(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
std::move_iterator<T1> f42(std::iter_difference_t<T1> a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f43(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
