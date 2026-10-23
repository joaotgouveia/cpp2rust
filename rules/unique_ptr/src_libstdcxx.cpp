#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f44() {
  return std::unique_ptr<T1[], T2>();
}

template <
    typename T1, typename T2, typename T3, typename T4 = T2,
    typename T5 = typename std::enable_if<std::integral_constant<
        bool,
        std::integral_constant<bool, !std::is_pointer<T2>::value>::value &&
            std::is_default_constructible<T2>::value>::value>::type,
    typename T6 = typename std::enable_if<
        std::__and_<std::__or_<
            std::__or_<std::is_same<
                           T3, typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                       std::is_same<T3, std::nullptr_t>>,
            std::__and_<
                std::is_pointer<T3>,
                std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer,
                             T1 *>,
                std::is_convertible<
                    typename std::remove_pointer<T3>::type (*)[], T1 (*)[]>>>>::
            value,
        bool>::type>
std::unique_ptr<T1[], T2> f45(T3 a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4 = T2,
    typename T5 = std::_Require<
        std::__and_<std::__or_<
            std::__or_<std::is_same<
                           T3, typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                       std::is_same<T3, std::nullptr_t>>,
            std::__and_<
                std::is_pointer<T3>,
                std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer,
                             T1 *>,
                std::is_convertible<
                    typename std::remove_pointer<T3>::type (*)[], T1 (*)[]>>>>,
        std::is_copy_constructible<T2>>>
std::unique_ptr<T1[], T2> f46(T3 a0, const T2 &a1) {
  return std::unique_ptr<T1[], T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1[], T2> f47(std::nullptr_t a0) {
  return std::unique_ptr<T1[], T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::__and_<
            std::is_array<T3>,
            std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer, T1 *>,
            std::is_same<typename std::unique_ptr<T3, T4>::pointer,
                         typename std::unique_ptr<T3, T4>::element_type *>,
            std::is_convertible<
                typename std::unique_ptr<T3, T4>::element_type (*)[],
                T1 (*)[]>>,
        std::__conditional_t<std::is_reference<T2>::value, std::is_same<T4, T2>,
                             std::is_convertible<T4, T2>>>>
std::unique_ptr<T1[], T2> f48(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::_Require<std::__or_<
        std::is_same<T3, typename std::__uniq_ptr_impl<T1, T2>::pointer>,
        std::__and_<
            std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer, T1 *>,
            std::is_pointer<T3>,
            std::is_convertible<typename std::remove_pointer<T3>::type (*)[],
                                T1 (*)[]>>>>>
void f49(std::unique_ptr<T1[], T2> &a0, T3 a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<T1>::type
f50(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f51() {
  return std::unique_ptr<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f52(T1 *a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = std::_Require<std::is_copy_constructible<T2>>>
std::unique_ptr<T1, T2> f53(T1 *a0, const T2 &a1) {
  return std::unique_ptr<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3 = T2,
          typename T4 = typename std::enable_if<std::integral_constant<
              bool, std::integral_constant<
                        bool, !std::is_pointer<T2>::value>::value &&
                        std::is_default_constructible<T2>::value>::value>::type>
std::unique_ptr<T1, T2> f54(std::nullptr_t a0) {
  return std::unique_ptr<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::__and_<
            std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
            std::__not_<std::is_array<T3>>>,
        std::__conditional_t<std::is_reference<T2>::value, std::is_same<T4, T2>,
                             std::is_convertible<T4, T2>>>>
std::unique_ptr<T1, T2> f55(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_Require<
              std::is_convertible<
                  T3 *, typename std::__uniq_ptr_impl<T1, T2>::pointer>,
              std::is_same<T2, std::default_delete<T1>>>>
std::unique_ptr<T1, T2> f56(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}
