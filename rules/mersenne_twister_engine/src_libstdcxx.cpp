#include <algorithm>
#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, std::size_t T3, std::size_t T4, std::size_t T5,
    T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10, std::size_t T11,
    T1 T12, std::size_t T13, T1 T14, typename T15,
    typename T16 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T15>::type>::type,
                          std::mersenne_twister_engine<
                              T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                              T13, T14>>::value>::value &&
                std::is_unsigned<typename T15::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T15, T1>::value>::value>::value,
        void>::type>
std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                             T13, T14>
f23(T15 &a0) {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15>
typename std::enable_if<
    std::integral_constant<
        bool,
        std::integral_constant<
            bool,
            !std::is_same<typename std::remove_cv<
                              typename std::remove_reference<T15>::type>::type,
                          std::mersenne_twister_engine<
                              T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                              T13, T14>>::value>::value &&
            std::is_unsigned<typename T15::result_type>::value &&
            std::integral_constant<
                bool, !std::is_convertible<T15, T1>::value>::value>::value,
    void>::type
f24(std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,
                                 T12, T13, T14> &a0,
    T15 &a1) {
  return a0.template seed<T15>(a1);
}
