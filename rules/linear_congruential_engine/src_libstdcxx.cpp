#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <
    typename T1, T1 T2, T1 T3, T1 T4, typename T5,
    typename T6 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T5>::type>::type,
                          std::linear_congruential_engine<T1, T2, T3,
                                                          T4>>::value>::value &&
                std::is_unsigned<typename T5::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T5, T1>::value>::value>::value,
        void>::type>
std::linear_congruential_engine<T1, T2, T3, T4> f13(T5 &a0) {
  return std::linear_congruential_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, T1 T2, T1 T3, T1 T4, typename T5>
typename std::enable_if<
    std::integral_constant<
        bool, std::integral_constant<
                  bool, !std::is_same<
                            typename std::remove_cv<
                                typename std::remove_reference<T5>::type>::type,
                            std::linear_congruential_engine<T1, T2, T3, T4>>::
                            value>::value &&
                  std::is_unsigned<typename T5::result_type>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<T5, T1>::value>::value>::value,
    void>::type
f14(std::linear_congruential_engine<T1, T2, T3, T4> &a0, T5 &a1) {
  return a0.template seed<T5>(a1);
}
