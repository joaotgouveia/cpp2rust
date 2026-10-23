#include <scoped_allocator>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, const T3 &>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f25(const std::scoped_allocator_adaptor<T3, T2...> &a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::scoped_allocator_adaptor<T1, T2...>(a0);
}

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<
              std::is_constructible<T1, T3>::value>::type>
std::scoped_allocator_adaptor<T1, T2...>
f26(std::scoped_allocator_adaptor<T3, T2...> &&a0)
  requires(!(std::is_same<T3, T1>::value))
{
  return std::scoped_allocator_adaptor<T1, T2...>(std::move(a0));
}
