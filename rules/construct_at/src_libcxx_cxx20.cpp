#include <memory>
#include <utility>

template <typename T1, typename... T2,
          typename T3 = decltype(::new (std::declval<void *>())
                                     T1(std::declval<T2>()...))>
T1 *f2(T1 *a0, T2 &&...a1) {
  return std::construct_at<T1, T2...>(a0, std::forward<T2>(a1)...);
}
