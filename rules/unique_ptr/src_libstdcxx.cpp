#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1[], T2>::element_type>::type
f43(const std::unique_ptr<T1[], T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f44() {
  return std::unique_ptr<T1[], T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f45(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1, T2>::element_type>::type
f46(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f47() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f48(typename std::unique_ptr<T1, T2>::pointer a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::unique_ptr<T1, T2>::deleter_type,
          typename T4 = typename std::enable_if<
              std::integral_constant<
                  bool, std::is_copy_constructible<T2>::value>::value,
              void>::type>
std::unique_ptr<T1, T2>
f49(typename std::unique_ptr<T1, T2>::pointer a0,
    const typename std::unique_ptr<T1, T2>::deleter_type &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f50(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}
