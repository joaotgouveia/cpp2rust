#include <map>
#include <utility>

template <typename T1>
bool f29(const std::_Rb_tree_const_iterator<T1> &a0,
         const std::_Rb_tree_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f30(const std::_Rb_tree_iterator<T1> &a0,
         const std::_Rb_tree_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <bool T1, typename T2, bool T3,
          typename T4 = std::__enable_if_t<T1 && !T3>,
          typename std::enable_if<(!(T3 == T1)), int>::type T5 = 0>
std::__rb_tree::_Iterator<T1, T2>
f31(const std::__rb_tree::_Iterator<T3, T2> &a0) {
  return std::__rb_tree::_Iterator<T1, T2>(a0);
}

template <bool T1, typename T2>
bool f32(const std::__rb_tree::_Iterator<T1, T2> &a0,
         const std::__rb_tree::_Iterator<T1, T2> &a1) {
  return operator!=(a0, a1);
}
