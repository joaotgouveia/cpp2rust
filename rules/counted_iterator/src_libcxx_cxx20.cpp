#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
decltype(auto) f30(const std::counted_iterator<T1> &a0)
  requires(std::__dereferenceable<const T1>)
{
  return a0.operator*();
}

template <typename T1> decltype(auto) f31(const std::counted_iterator<T1> &a0) {
  return iter_move(a0);
}
