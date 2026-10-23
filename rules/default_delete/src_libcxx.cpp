#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_convertible<T2 (*)[], T1 (*)[]>::value, int>::type T3 = 0>
std::default_delete<T1[]> f7(const std::default_delete<T2[]> &a0) {
  return std::default_delete<T1[]>(a0);
}

template <typename T1, typename T2,
          typename std::enable_if<
              std::is_convertible<T2 (*)[], T1 (*)[]>::value, int>::type T3 = 0>
void f8(const std::default_delete<T1[]> &a0, T2 *a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2,
          typename std::enable_if<std::is_convertible<T2 *, T1 *>::value,
                                  int>::type T3 = 0>
std::default_delete<T1> f9(const std::default_delete<T2> &a0) {
  return std::default_delete<T1>(a0);
}
