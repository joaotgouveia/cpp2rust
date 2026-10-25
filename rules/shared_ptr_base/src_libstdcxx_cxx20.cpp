#include <algorithm>
#include <barrier>
#include <chrono>
#include <utility>

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f86(const std::__shared_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f87(std::__shared_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f88(std::__shared_ptr<T1, T2> &a0, const std::__shared_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f89(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
T1 *f90(const std::__shared_ptr<T1, T2> &a0)
  requires(T2 == T2) && (std::is_array<T1>::value == false) &&
          (std::is_void<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1, std::_Lock_policy T2>
typename std::__shared_ptr<T1, T2>::element_type &
f91(const std::__shared_ptr<T1, T2> &a0, std::ptrdiff_t a1)
  requires(T2 == T2) && (std::is_array<T1>::value == true) &&
          (std::is_void<T1>::value == false)
{
  return a0.operator[](a1);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__weak_ptr<T1, T2> f92(const std::__weak_ptr<T3, T2> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__weak_ptr<T1, T2> f93(std::__weak_ptr<T3, T2> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f94(std::__weak_ptr<T1, T2> &a0, const std::__weak_ptr<T3, T2> &a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f95(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T3, T2> &&a1)
  requires(!(std::is_same<T3, T1>::value))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
std::strong_ordering f96(const std::__shared_ptr<T1, T2> &a0,
                         std::nullptr_t a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::strong_ordering f97(const std::__shared_ptr<T1, T3> &a0,
                         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
