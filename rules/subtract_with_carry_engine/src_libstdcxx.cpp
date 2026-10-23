#include <algorithm>
#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <
    typename T1, std::size_t T2, std::size_t T3, std::size_t T4, typename T5,
    typename T6 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T5>::type>::type,
                          std::subtract_with_carry_engine<T1, T2, T3,
                                                          T4>>::value>::value &&
                std::is_unsigned<typename T5::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T5, T1>::value>::value>::value,
        void>::type>
std::subtract_with_carry_engine<T1, T2, T3, T4> f12(T5 &a0) {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5>
typename std::enable_if<
    std::integral_constant<
        bool, std::integral_constant<
                  bool, !std::is_same<
                            typename std::remove_cv<
                                typename std::remove_reference<T5>::type>::type,
                            std::subtract_with_carry_engine<T1, T2, T3, T4>>::
                            value>::value &&
                  std::is_unsigned<typename T5::result_type>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<T5, T1>::value>::value>::value,
    void>::type
f13(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0, T5 &a1) {
  return a0.template seed<T5>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::uint_least32_t f14() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::default_seed;
}
