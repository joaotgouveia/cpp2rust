#include <algorithm>
#include <array>
#include <cstddef>
#include <span>
#include <utility>

template <typename T1, std::size_t T2>
std::span<const std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f16(std::span<T1, T2> a0) {
  return std::as_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

template <typename T1, std::size_t T2>
std::span<std::byte,
          T2 == std::dynamic_extent ? std::dynamic_extent : T2 * sizeof(T1)>
f17(std::span<T1, T2> a0)
  requires((!std::is_const_v<T1>))
{
  return std::as_writable_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f18()
  requires((T2 == std::dynamic_extent || T2 == 0))
{
  return std::span<T1, T2>();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f19(const std::span<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::iterator f20(const std::span<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f21(const std::span<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
typename std::span<T1, T2>::reverse_iterator f22(const std::span<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f23(T3 a0, std::size_t a1)
  requires(std::contiguous_iterator<T3>)
{
  return std::span<T1, T2>(std::move(a0), a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::span<T1, T2> f24(T3 a0, T4 a1)
  requires(std::contiguous_iterator<T3>) && (std::sized_sentinel_for<T4, T3>)
{
  return std::span<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, std::size_t T2, std::size_t T3>
std::span<T1, T2> f25(typename std::type_identity<T1>::type (&a0)[T3])
  requires((T2 == std::dynamic_extent || T3 == T2))
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f26(std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f27(const std::array<T3, T4> &a0) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f28(T3 &&a0)
  requires(!(std::is_same<T3, const std::span<T1, T2> &>::value) &&
           !(std::is_same<T3, typename std::span<T1, T2>::_SizedPtr>::value))
{
  return std::span<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, std::size_t T2, typename T3, std::size_t T4>
std::span<T1, T2> f29(const std::span<T3, T4> &a0)
  requires((T2 == std::dynamic_extent || T4 == std::dynamic_extent ||
            T2 == T4) &&
           (std::__is_array_convertible<T1, T3>::value)) &&
          (!(std::is_same<T3, T1>::value && T4 == T2))
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3,
          std::size_t T4 = std::dynamic_extent>
auto f30(const std::span<T1, T2> &a0) {
  return a0.template subspan<T3>();
}
