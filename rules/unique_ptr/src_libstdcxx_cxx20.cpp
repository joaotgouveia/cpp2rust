#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> std::unique_ptr<T1> f62() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1> auto f63(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f64(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
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
std::unique_ptr<T1[], T2> f65(T3 a0)
  requires(!(std::is_same<T3, std::unique_ptr<T1[], T2> &&>::value) &&
           !(std::is_same<T3, std::nullptr_t>::value) &&
           !(std::is_same<T3, const std::unique_ptr<T1[], T2> &>::value))
{
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
                             std::is_convertible<T4, T2>>>>
std::unique_ptr<T1[], T2> f66(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
  return std::unique_ptr<T1[], T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
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
f67(std::unique_ptr<T1[], T2> &a0, std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
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
                                T1 (*)[]>>>>>
void f68(std::unique_ptr<T1[], T2> &a0, T3 a1)
  requires(!(std::is_same<T3, std::nullptr_t>::value))
{
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
                             std::is_convertible<T4, T2>>>>
std::unique_ptr<T1, T2> f69(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<
    std::__and_<std::__and_<std::is_convertible<
                                typename std::unique_ptr<T3, T4>::pointer,
                                typename std::__uniq_ptr_impl<T1, T2>::pointer>,
                            std::__not_<std::is_array<T3>>>,
                std::is_assignable<T2 &, T4 &&>>::value,
    std::unique_ptr<T1, T2> &>::type
f70(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}
