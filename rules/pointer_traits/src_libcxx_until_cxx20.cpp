#include <memory>
#include <utility>

template <typename T1,
          typename std::enable_if<(std::integral_constant<bool, true>::value),
                                  int>::type T2 = 0>
T1 f8(typename std::conditional<
      std::is_void<
          typename std::__pointer_traits_element_type<T1>::type>::value,
      std::__nat, typename std::__pointer_traits_element_type<T1>::type>::type
          &a0) {
  return std::pointer_traits<T1>::pointer_to(a0);
}
