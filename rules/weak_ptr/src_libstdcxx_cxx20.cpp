#include <chrono>
#include <utility>

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, const std::weak_ptr<T2> &>::value>::type>
std::weak_ptr<T1> f18(const std::weak_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, std::weak_ptr<T2>>::value>::type>
std::weak_ptr<T1> f19(std::weak_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::weak_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
typename std::enable_if<
    std::is_assignable<std::__weak_ptr<T1> &, const std::weak_ptr<T2> &>::value,
    std::weak_ptr<T1> &>::type
f20(std::weak_ptr<T1> &a0, const std::weak_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
typename std::enable_if<
    std::is_assignable<std::__weak_ptr<T1> &, std::weak_ptr<T2>>::value,
    std::weak_ptr<T1> &>::type
f21(std::weak_ptr<T1> &a0, std::weak_ptr<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}
