#include <algorithm>
#include <forward_list>
#include <utility>

template <bool T1, typename T2>
std::__fwdlist::_Iterator<T1, T2>
f103(const std::__fwdlist::_Iterator<false, T2> &a0)
  requires(T1)
{
  return std::__fwdlist::_Iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::size_t f104(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
std::size_t f105(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
std::size_t f106(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::size_t f107(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(std::move(a1));
}
