#include <iterator>
#include <utility>

template <typename T1, typename T2>
std::common_iterator<T1, T2> f24()
  requires(std::default_initializable<T1>) &&
          ((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return std::common_iterator<T1, T2>();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f25(T1 a0)
  requires((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f26(T2 a0)
  requires((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
decltype(auto) f27(std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return a0.operator*();
}

template <typename T1, typename T2>
decltype(auto) f28(const std::common_iterator<T1, T2> &a0)
  requires(std::__dereferenceable<const T1>) &&
          ((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return a0.operator*();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f29(std::common_iterator<T1, T2> &a0)
  requires((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return a0.operator++();
}

template <typename T1, typename T2>
decltype(auto) f30(std::common_iterator<T1, T2> &a0, int a1)
  requires((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> f31(const std::common_iterator<T3, T4> &a0)
  requires(std::convertible_to<const T3 &, T1> &&
           std::convertible_to<const T4 &, T2>) &&
          ((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> &f32(std::common_iterator<T1, T2> &a0,
                                  const std::common_iterator<T3, T4> &a1)
  requires(std::convertible_to<const T3 &, T1> &&
           std::convertible_to<const T4 &, T2> &&
           std::assignable_from<T1 &, const T3 &> &&
           std::assignable_from<T2 &, const T4 &>) &&
          ((!std::same_as<T1, T2> && std::copyable<T1>))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3 = T1>
auto f33(const std::common_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
decltype(auto) f34(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}
