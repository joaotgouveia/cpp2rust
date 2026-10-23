#include <condition_variable>
#include <utility>

template <typename T1, typename T2,
          typename T3 = std::_Require<std::is_convertible<T2 (*)[], T1 (*)[]>>>
std::default_delete<T1[]> f4(const std::default_delete<T2[]> &a0) {
  return std::default_delete<T1[]>(a0);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_convertible<T2 (*)[], T1 (*)[]>::value>::type
f5(const std::default_delete<T1[]> &a0, T2 *a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2,
          typename T3 = std::_Require<std::is_convertible<T2 *, T1 *>>>
std::default_delete<T1> f6(const std::default_delete<T2> &a0) {
  return std::default_delete<T1>(a0);
}
