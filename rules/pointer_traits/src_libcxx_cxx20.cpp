#include <memory>
#include <utility>

template <typename T1>
T1 f9(typename std::conditional<
      std::is_void<
          typename std::__pointer_traits_element_type<T1>::type>::value,
      std::__nat, typename std::__pointer_traits_element_type<T1>::type>::type
          &a0)
  requires(std::integral_constant<bool, true>::value)
{
  return std::pointer_traits<T1>::pointer_to(a0);
}

template <typename T1> auto f10(const T1 &a0) {
  return std::to_address<T1>(a0);
}

template <typename T1> auto f11(T1 *a0) { return std::to_address<T1>(a0); }
