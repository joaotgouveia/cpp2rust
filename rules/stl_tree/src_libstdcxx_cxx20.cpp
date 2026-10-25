#include <map>
#include <utility>

template <bool T1, typename T2>
std::__rb_tree::_Iterator<T1, T2>
f33(const std::__rb_tree::_Iterator<false, T2> &a0)
  requires(T1)
{
  return std::__rb_tree::_Iterator<T1, T2>(a0);
}
