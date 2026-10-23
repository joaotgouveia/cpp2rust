#include <algorithm>
#include <utility>

template <typename T1>
decltype(auto) f24(const std::counted_iterator<T1> &a0)
  requires(std::__detail::__dereferenceable<const T1>)
{
  return a0.operator*();
}

template <typename T1>
std::iter_rvalue_reference_t<T1> f25(const std::counted_iterator<T1> &a0) {
  return iter_move(a0);
}
