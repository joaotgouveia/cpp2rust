#include <chrono>
#include <utility>

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, const std::weak_ptr<T2> &>::value>::type>
std::weak_ptr<T1> f13(const std::weak_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, std::weak_ptr<T2>>::value>::type>
std::weak_ptr<T1> f14(std::weak_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::weak_ptr<T1>(std::move(a0));
}
