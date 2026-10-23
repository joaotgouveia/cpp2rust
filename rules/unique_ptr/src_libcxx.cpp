#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_swappable_v<T2>, int> T3 = 0>
void f62(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f63() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f64(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4 = T2,
    std::__enable_if_t<std::is_default_constructible<T2>::value &&
                           !std::is_pointer<T2>::value,
                       int>
        T5 = 0,
    std::__enable_if_t<std::unique_ptr<T1[], T2>::
                           template _CheckArrayPointerConversion<T3>::value,
                       int>
        T6 = 0>
std::unique_ptr<T1[], T2> f65(T3 a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::is_same<T3, std::__private_constructor_tag>::value, int>
              T5 = 0>
std::unique_ptr<T1[], T2> f66(T3 a0, T4 a1, std::size_t a2) {
  return std::unique_ptr<T1[], T2>(a0, a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4 = T2,
    std::__enable_if_t<std::is_copy_constructible<T2>::value, int> T5 = 0,
    std::__enable_if_t<std::unique_ptr<T1[], T2>::
                           template _CheckArrayPointerConversion<T3>::value,
                       int>
        T6 = 0>
std::unique_ptr<T1[], T2> f67(T3 a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_copy_constructible<T2>::value, int> T4 = 0>
std::unique_ptr<T1[], T2> f68(std::nullptr_t a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4 = T2,
    std::__enable_if_t<!std::is_reference<T2>::value &&
                           std::is_move_constructible<T2>::value,
                       int>
        T5 = 0,
    std::__enable_if_t<std::unique_ptr<T1[], T2>::
                           template _CheckArrayPointerConversion<T3>::value,
                       int>
        T6 = 0>
std::unique_ptr<T1[], T2> f69(T3 a0, T2 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<!std::is_reference<T2>::value &&
                                 std::is_move_constructible<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1[], T2> f70(std::nullptr_t a0, T2 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<
        std::is_array<T3>::value &&
        std::is_same<std::__pointer<T1, T2>, T1 *>::value &&
        std::is_same<typename std::unique_ptr<T3, T4>::pointer,
                     typename std::unique_ptr<T3, T4>::element_type *>::value &&
        std::is_convertible<
            typename std::unique_ptr<T3, T4>::element_type (*)[],
            T1 (*)[]>::value>,
    typename T6 = std::__enable_if_t<
        (std::is_reference<T2>::value && std::is_same<T2, T4>::value) ||
        (!std::is_reference<T2>::value && std::is_convertible<T4, T2>::value)>>
std::unique_ptr<T1[], T2> f71(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<
        std::is_array<T3>::value &&
        std::is_same<std::__pointer<T1, T2>, T1 *>::value &&
        std::is_same<typename std::unique_ptr<T3, T4>::pointer,
                     typename std::unique_ptr<T3, T4>::element_type *>::value &&
        std::is_convertible<
            typename std::unique_ptr<T3, T4>::element_type (*)[],
            T1 (*)[]>::value>,
    typename T6 = std::__enable_if_t<std::is_assignable<T2 &, T4 &&>::value>>
std::unique_ptr<T1[], T2> &f72(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::unique_ptr<T1[], T2>::
                           template _CheckArrayPointerConversion<T3>::value,
                       int>
        T4 = 0>
void f73(std::unique_ptr<T1[], T2> &a0, T3 a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f74() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f75(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_default_constructible<T2>::value &&
                                 !std::is_pointer<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f76(T1 *a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<std::is_copy_constructible<T2>::value, int> T4 = 0>
std::unique_ptr<T1, T2> f77(T1 *a0, const T2 &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          std::__enable_if_t<!std::is_reference<T2>::value &&
                                 std::is_move_constructible<T2>::value,
                             int>
              T4 = 0>
std::unique_ptr<T1, T2> f78(T1 *a0, T2 &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<
        std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                            std::__pointer<T1, T2>>::value &&
        !std::is_array<T3>::value>,
    typename T6 = std::__enable_if_t<
        (std::is_reference<T2>::value && std::is_same<T2, T4>::value) ||
        (!std::is_reference<T2>::value && std::is_convertible<T4, T2>::value)>>
std::unique_ptr<T1, T2> f79(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<
        std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                            std::__pointer<T1, T2>>::value &&
        !std::is_array<T3>::value>,
    typename T6 = std::__enable_if_t<std::is_assignable<T2 &, T4 &&>::value>>
std::unique_ptr<T1, T2> &f80(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3 = T1 *,
    std::__enable_if_t<std::__can_dereference<std::__pointer<T1, T2>>, int> T4 =
        0>
typename std::add_lvalue_reference<T1>::type
f81(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}
