#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f57(const std::unique_ptr<T1[], T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1[], T2>::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f58() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1[], T2>::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2> f59(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1[], T2>::deleter_type,
          typename std::enable_if<std::is_copy_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2>
f60(std::nullptr_t a0,
    const typename std::unique_ptr<T1[], T2>::deleter_type &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1[], T2>::deleter_type,
          typename std::enable_if<!std::is_reference<T2>::value &&
                                      std::is_move_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1[], T2>
f61(std::nullptr_t a0, typename std::unique_ptr<T1[], T2>::deleter_type &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f62() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f63(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename std::enable_if<std::is_default_constructible<T2>::value &&
                                      !std::is_pointer<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2> f64(typename std::unique_ptr<T1, T2>::pointer a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename std::enable_if<std::is_copy_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2>
f65(typename std::unique_ptr<T1, T2>::pointer a0,
    const typename std::unique_ptr<T1, T2>::deleter_type &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename std::enable_if<!std::is_reference<T2>::value &&
                                      std::is_move_constructible<T2>::value,
                                  int>::type T4 = 0>
std::unique_ptr<T1, T2>
f66(typename std::unique_ptr<T1, T2>::pointer a0,
    typename std::unique_ptr<T1, T2>::deleter_type &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}
