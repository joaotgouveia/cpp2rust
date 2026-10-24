#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1, typename T2> using t1 = std::common_iterator<T1, T2>;

template <typename T1, typename T2, typename T3, typename T4>
void f1(const std::common_iterator<T1, T2> &a0,
        const std::common_iterator<T3, T4> &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::iter_difference_t<T3> f2(const std::common_iterator<T1, T2> &a0,
                              const std::common_iterator<T3, T4> &a1)
  requires(std::sized_sentinel_for<T2, T3>)
{
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f3(const std::common_iterator<T1, T2> &a0,
        const std::common_iterator<T3, T4> &a1)
  requires(std::sentinel_for<T2, T3> && std::equality_comparable_with<T1, T3>)
{
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f4(const std::common_iterator<T1, T2> &a0,
        const std::common_iterator<T3, T4> &a1)
  requires(std::sentinel_for<T2, T3>)
{
  return operator==(a0, a1);
}
