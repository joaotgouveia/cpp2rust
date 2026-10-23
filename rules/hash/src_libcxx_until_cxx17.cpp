#include <algorithm>
#include <cstddef>
#include <functional>
#include <memory>
#include <system_error>
#include <typeindex>
#include <utility>
#include <vector>

template <typename T1, typename T2,
          typename std::enable_if<std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
typename std::hash<std::shared_ptr<T1>>::argument_type
f250(std::auto_ptr<T2> &&a0) {
  return typename std::hash<std::shared_ptr<T1>>::argument_type(std::move(a0));
}

template <typename T1, typename T2,
          typename std::enable_if<!std::is_array<T2>::value &&
                                      std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
typename std::hash<std::shared_ptr<T1>>::argument_type &
f251(typename std::hash<std::shared_ptr<T1>>::argument_type &a0,
     std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
typename std::hash<std::__enable_hash_helper<
    std::unique_ptr<T1, T2>,
    typename std::unique_ptr<T1, T2>::pointer>>::argument_type
f252(std::auto_ptr<T3> &&a0) {
  return typename std::hash<std::__enable_hash_helper<
      std::unique_ptr<T1, T2>,
      typename std::unique_ptr<T1, T2>::pointer>>::argument_type(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
typename std::hash<std::unique_ptr<T1, T2>>::argument_type &
f253(typename std::hash<std::__enable_hash_helper<
         std::unique_ptr<T1, T2>,
         typename std::unique_ptr<T1, T2>::pointer>>::argument_type &a0,
     std::auto_ptr<T3> a1) {
  return a0.template operator= <T3>(a1);
}
