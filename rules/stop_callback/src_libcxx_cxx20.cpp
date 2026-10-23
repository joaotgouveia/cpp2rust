#include <stop_token>
#include <utility>

template <typename T1, typename T2>
std::stop_callback<T1> f3(const std::stop_token &a0, T2 &&a1)
  requires(std::constructible_from<T1, T2>)
{
  return std::stop_callback<T1>(a0, std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::stop_callback<T1> f4(std::stop_token &&a0, T2 &&a1)
  requires(std::constructible_from<T1, T2>)
{
  return std::stop_callback<T1>(std::move(a0), std::forward<T2>(a1));
}
