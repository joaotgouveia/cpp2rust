#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::ranges::borrowed_subrange_t<T1>;

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f1(std::ranges::subrange<T2, T3, T4> &&a0)
  requires((T1 < 2))
{
  return std::ranges::get<T1>(std::move(a0));
}

template <std::size_t T1, typename T2, typename T3,
          std::ranges::subrange_kind T4>
auto f2(const std::ranges::subrange<T2, T3, T4> &a0)
  requires(((T1 == 0 && std::copyable<T2>) || T1 == 1))
{
  return std::ranges::get<T1>(a0);
}

template <typename T1, typename T2>
using t2 = std::ranges::mismatch_result<T1, T2>;

using t3 = std::ranges::subrange_kind;

std::ranges::subrange_kind f3() { return std::ranges::subrange_kind::unsized; }

std::ranges::subrange_kind f4() { return std::ranges::subrange_kind::sized; }
