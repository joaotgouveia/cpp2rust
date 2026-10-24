#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f124(std::vector<T1, T2> &a0,
                                             const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::size_type f125(std::vector<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::iterator
f126(const __gnu_cxx::__normal_iterator<T3, std::vector<T1, T2>> &a0)
  requires(std::is_convertible_v<T3, typename std::vector<T1, T2>::pointer>)
{
  return typename std::vector<T1, T2>::iterator(a0);
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::const_iterator
f127(const __gnu_cxx::__normal_iterator<T3, std::vector<T1, T2>> &a0)
  requires(
      std::is_convertible_v<T3, typename std::vector<T1, T2>::const_pointer>)
{
  return typename std::vector<T1, T2>::const_iterator(a0);
}
