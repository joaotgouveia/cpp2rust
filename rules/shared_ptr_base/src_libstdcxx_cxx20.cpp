#include <algorithm>
#include <barrier>
#include <chrono>
#include <utility>

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f82(const std::__shared_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f83(std::__shared_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f84(std::__shared_ptr<T1, T2> &a0, const std::__shared_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f85(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__weak_ptr<T1, T2> f86(const std::__weak_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__weak_ptr<T1, T2> f87(std::__weak_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f88(std::__weak_ptr<T1, T2> &a0, const std::__weak_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f89(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
std::strong_ordering f90(const std::__shared_ptr<T1, T2> &a0,
                         std::nullptr_t a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::strong_ordering f91(const std::__shared_ptr<T1, T3> &a0,
                         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
