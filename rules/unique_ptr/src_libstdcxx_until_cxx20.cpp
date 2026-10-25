#include <algorithm>
#include <condition_variable>
#include <utility>

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
        bool>::type,
    typename std::enable_if<
        (!(std::is_same<T3, std::unique_ptr<T1[], T2> &&>::value) &&
         !(std::is_same<T3, std::nullptr_t>::value) &&
         !(std::is_same<T3, const std::unique_ptr<T1[], T2> &>::value)),
        int>::type T7 = 0>
std::unique_ptr<T1[], T2> f55(T3 a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
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
                             std::is_convertible<T4, T2>>>,
    typename std::enable_if<(!(std::is_same<T3, T1[]>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::unique_ptr<T1[], T2> f56(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<(!(std::is_same<T3, T1[]>::value &&
                                     std::is_same<T4, T2>::value)),
                                  int>::type T5 = 0>
typename std::enable_if<
    std::__and_<
        std::__and_<
            std::is_array<T3>,
            std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer, T1 *>,
            std::is_same<typename std::unique_ptr<T3, T4>::pointer,
                         typename std::unique_ptr<T3, T4>::element_type *>,
            std::is_convertible<
                typename std::unique_ptr<T3, T4>::element_type (*)[],
                T1 (*)[]>>,
        std::is_assignable<T2 &, T4 &&>>::value,
    std::unique_ptr<T1[], T2> &>::type
f57(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::_Require<std::__or_<
        std::is_same<T3, typename std::__uniq_ptr_impl<T1, T2>::pointer>,
        std::__and_<
            std::is_same<typename std::__uniq_ptr_impl<T1, T2>::pointer, T1 *>,
            std::is_pointer<T3>,
            std::is_convertible<typename std::remove_pointer<T3>::type (*)[],
                                T1 (*)[]>>>>,
    typename std::enable_if<(!(std::is_same<T3, std::nullptr_t>::value)),
                            int>::type T5 = 0>
void f58(std::unique_ptr<T1[], T2> &a0, T3 a1) {
  return a0.template reset<T3>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::__and_<
            std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
            std::__not_<std::is_array<T3>>>,
        std::__conditional_t<std::is_reference<T2>::value, std::is_same<T4, T2>,
                             std::is_convertible<T4, T2>>>,
    typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                               std::is_same<T4, T2>::value)),
                            int>::type T6 = 0>
std::unique_ptr<T1, T2> f59(std::unique_ptr<T3, T4> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<(!(std::is_same<T3, T1>::value &&
                                     std::is_same<T4, T2>::value)),
                                  int>::type T5 = 0>
typename std::enable_if<
    std::__and_<std::__and_<std::is_convertible<
                                typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                            std::__not_<std::is_array<T3>>>,
                std::is_assignable<T2 &, T4 &&>>::value,
    std::unique_ptr<T1, T2> &>::type
f60(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}
