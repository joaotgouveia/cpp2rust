#include <algorithm>
#include <utility>

template <typename T1,
          typename std::enable_if<(std::is_same<T1 *, T1 *>::value) &&
                                      (std::is_void<T1>::value == false),
                                  int>::type T2 = 0>
T1 *f3(T1 &a0) {
  return std::pointer_traits<T1 *>::pointer_to(a0);
}

template <typename T1,
          typename std::enable_if<
              (std::is_same<T1, std::__ptr_traits_elem_t<T1> *>::value) &&
                  (std::is_void<std::__ptr_traits_elem_t<T1>>::value == false),
              int>::type T2 = 0>
auto f4(typename std::pointer_traits<T1>::element_type &a0)
    -> decltype(std::pointer_traits<T1>::pointer_to(a0)) {
  return std::pointer_traits<T1>::pointer_to(a0);
}
