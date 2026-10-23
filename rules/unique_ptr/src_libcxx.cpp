#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f57() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f58(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_copy_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f59(std::nullptr_t a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<!std::is_reference<T2>::value &&
                                      std::is_move_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f60(std::nullptr_t a0, T2 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f61() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f62(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f63(T1 *a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<std::is_copy_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f64(T1 *a0, const T2 &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename std::enable_if<!std::is_reference<T2>::value &&
                                      std::is_move_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f65(T1 *a0, T2 &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}
