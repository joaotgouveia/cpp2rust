#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
std::move_iterator<T1> f46()
  requires(std::default_initializable<T1>)
{
  return std::move_iterator<T1>();
}

template <typename T1>
auto f47(std::move_iterator<T1> &a0, int a1)
  requires(std::forward_iterator<T1>)
{
  return a0.operator++(a1);
}

template <typename T1> void f48(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f49(const std::move_iterator<T2> &a0)
  requires((!std::_IsSame<T2, T1>::value) &&
           std::convertible_to<const T2 &, T1>)
{
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f50(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1)
  requires((!std::_IsSame<T2, T1>::value) &&
           std::convertible_to<const T2 &, T1> &&
           std::assignable_from<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
bool f51(const std::move_iterator<T1> &a0, const std::move_sentinel<T2> &a1)
  requires(std::__move_iter_comparable<T1, T2>) && (std::sentinel_for<T2, T1>)
{
  return operator==(a0, a1);
}

template <typename T1>
std::move_iterator<T1> f52(std::iter_difference_t<T1> a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f53(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}
