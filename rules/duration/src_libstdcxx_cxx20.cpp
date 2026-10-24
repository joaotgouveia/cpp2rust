#include <chrono>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::_Require<
        std::is_convertible<const T3 &, T1>,
        std::__or_<std::chrono::treat_as_floating_point<T1>,
                   std::__not_<std::chrono::treat_as_floating_point<T3>>>>>
std::chrono::duration<T1, T2> f22(const T3 &a0)
  requires(!(std::is_same<T3, std::chrono::duration<T1, T2>>::value))
{
  return std::chrono::duration<T1, T2>(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::is_convertible<const T3 &, T1>,
        std::__or_<
            std::chrono::treat_as_floating_point<T1>,
            std::__and_<
                std::__bool_constant<
                    std::ratio<
                        (T4::num / std::chrono::duration<T1, T2>::_S_gcd(
                                       T4::num, T2::num)) *
                            (T2::den / std::chrono::duration<T1, T2>::_S_gcd(
                                           T4::den, T2::den)),
                        (T4::den / std::chrono::duration<T1, T2>::_S_gcd(
                                       T4::den, T2::den)) *
                            (T2::num / std::chrono::duration<T1, T2>::_S_gcd(
                                           T4::num, T2::num))>::den == 1>,
                std::__not_<std::chrono::treat_as_floating_point<T3>>>>>>
std::chrono::duration<T1, T2> f23(const std::chrono::duration<T3, T4> &a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::chrono::duration<T1, T2>(a0);
}
