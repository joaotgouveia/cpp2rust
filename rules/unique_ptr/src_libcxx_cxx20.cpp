#include <compare>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, std::enable_if_t<!std::is_array_v<T1>, int> T2 = 0>
std::unique_ptr<T1> f88() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1,
          std::enable_if_t<std::is_unbounded_array_v<T1>, int> T2 = 0>
std::unique_ptr<T1> f89(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
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
std::unique_ptr<T1[], T2> f90(T3 a0)
  requires(!(std::is_same<T3, std::unique_ptr<T1[], T2> &&>::value))
{
  return std::unique_ptr<T1[], T2>(a0);
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
std::unique_ptr<T1[], T2> f91(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
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
std::unique_ptr<T1[], T2> &f92(std::unique_ptr<T1[], T2> &a0,
                               std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1[]>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::unique_ptr<T1[], T2>::
                           template _CheckArrayPointerConversion<T3>::value,
                       int>
        T4 = 0>
void f93(std::unique_ptr<T1[], T2> &a0, T3 a1)
  requires(!(std::is_same<T3, std::nullptr_t>::value))
{
  return a0.template reset<T3>(a1);
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
std::unique_ptr<T1, T2> f94(std::unique_ptr<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::unique_ptr<T1, T2>(std::move(a0));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<
        std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                            std::__pointer<T1, T2>>::value &&
        !std::is_array<T3>::value>,
    typename T6 = std::__enable_if_t<std::is_assignable<T2 &, T4 &&>::value>>
std::unique_ptr<T1, T2> &f95(std::unique_ptr<T1, T2> &a0,
                             std::unique_ptr<T3, T4> &&a1)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}
