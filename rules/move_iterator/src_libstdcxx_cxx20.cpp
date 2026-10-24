#include <algorithm>
#include <utility>

template <typename T1>
void f29(std::move_iterator<T1> &a0, int a1)
  requires((!std::forward_iterator<T1>))
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f30(const std::move_iterator<T2> &a0)
  requires(std::move_iterator<T1>::template __convertible<T2>)
{
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f31(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1)
  requires(std::move_iterator<T1>::template __convertible<T2> &&
           std::assignable_from<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
bool f32(const std::move_iterator<T1> &a0, const std::move_sentinel<T2> &a1)
  requires(std::sentinel_for<T2, T1>)
{
  return operator==(a0, a1);
}
