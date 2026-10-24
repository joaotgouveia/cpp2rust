#include <algorithm>
#include <utility>

template <typename T1>
using t7 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t8 = typename std::reverse_iterator<T1>::value_type;

template <typename T1, typename T2>
std::reverse_iterator<T1> f27(const std::reverse_iterator<T2> &a0)
  requires(std::reverse_iterator<T1>::template __convertible<T2>) &&
          (!(std::is_same<T2, T1>::value))
{
  return std::reverse_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::reverse_iterator<T1> &f28(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1)
  requires(std::reverse_iterator<T1>::template __convertible<T2> &&
           std::assignable_from<T1 &, const T2 &>) &&
          (!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}
