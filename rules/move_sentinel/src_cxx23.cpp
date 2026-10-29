#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::move_sentinel<T1>;

template <typename T1> std::move_sentinel<T1> f1() {
  return std::move_sentinel<T1>();
}

template <typename T1> std::move_sentinel<T1> f2(T1 a0) {
  return std::move_sentinel<T1>(std::move(a0));
}

template <typename T1> T1 f3(const std::move_sentinel<T1> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::move_sentinel<T1> f4(const std::move_sentinel<T2> &a0)
  requires(std::convertible_to<const T2 &, T1>)
{
  return std::move_sentinel<T1>(a0);
}

template <typename T1, typename T2>
std::move_sentinel<T1> &f5(std::move_sentinel<T1> &a0,
                           const std::move_sentinel<T2> &a1)
  requires(std::assignable_from<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}
