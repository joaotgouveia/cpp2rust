#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_swappable_v<T2>, int> T3 = 0>
void f71(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f72() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f73(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::is_same<T3, std::__private_constructor_tag>::value, int>
              T5 = 0>
std::unique_ptr<T1[], T2> f74(T3 a0, T4 a1, std::size_t a2) {
  return std::unique_ptr<T1[], T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_copy_constructible<T2>::value, int> T4 = 0>
std::unique_ptr<T1[], T2> f75(std::nullptr_t a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<!std::is_reference<T2>::value &&
                                 std::is_move_constructible<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f76(std::nullptr_t a0, T2 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f77() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f78(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f79(T1 *a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_copy_constructible<T2>::value, int> T4 = 0>
std::unique_ptr<T1, T2> f80(T1 *a0, const T2 &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<!std::is_reference<T2>::value &&
                                 std::is_move_constructible<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f81(T1 *a0, T2 &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}

template <
    typename T1, typename T2, typename T3 = T1 *,
    std::__enable_if_t<std::__can_dereference<std::__pointer<T1, T2>>, int> T4 =
        0>
typename std::add_lvalue_reference<T1>::type
f82(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}
