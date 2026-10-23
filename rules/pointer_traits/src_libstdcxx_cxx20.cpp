#include <algorithm>
#include <utility>

template <typename T1>
T1 *f5(T1 &a0)
  requires(std::is_same<T1 *, T1 *>::value) &&
          (std::is_void<T1>::value == false)
{
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1>
auto f6(typename std::pointer_traits<T1>::element_type &a0)
  requires(std::is_same<T1, std::__ptr_traits_elem_t<T1> *>::value) &&
          (std::is_void<std::__ptr_traits_elem_t<T1>>::value == false)
{
  return std::pointer_traits<T1>::pointer_to(a0);
}
