#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
std::unique_ptr<T1, T2> f67(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<T3 *, T1 *>::value &&
                  std::is_same<T2, std::default_delete<T1>>::value,
              int>::type T4 = 0>
std::unique_ptr<T1, T2> &f68(std::unique_ptr<T1, T2> &a0,
                             std::auto_ptr<T3> a1) {
  return a0.template operator= <T3>(a1);
}
