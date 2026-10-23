#include <compare>
#include <iterator>
#include <utility>

template <typename T1> const T1 &f36(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f37(std::move_iterator<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> auto f38(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f39(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f40(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
std::move_iterator<T1> f41(std::iter_difference_t<T1> a0,
                           const std::move_iterator<T1> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f42(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1) {
  return a0 <=> a1;
}
