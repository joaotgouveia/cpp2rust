#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
std::move_iterator<T1> f42()
  requires(std::default_initializable<T1>)
{
  return std::move_iterator<T1>();
}

template <typename T1>
auto f43(std::move_iterator<T1> &a0, int a1)
  requires(std::forward_iterator<T1>)
{
  return a0.operator++(a1);
}

template <typename T1> void f44(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f45(const std::move_iterator<T2> &a0)
  requires((!std::_IsSame<T2, T1>::value) &&
           std::convertible_to<const T2 &, T1>)
{
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f46(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1)
  requires((!std::_IsSame<T2, T1>::value) &&
           std::convertible_to<const T2 &, T1> &&
           std::assignable_from<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}

template <typename T1>
std::move_iterator<T1> f47(std::iter_difference_t<T1> a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f48(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
