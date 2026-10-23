#include <algorithm>
#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, typename T3,
    typename T4 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool, !std::is_same<
                          typename std::remove_cv<
                              typename std::remove_reference<T3>::type>::type,
                          std::shuffle_order_engine<T1, T2>>::value>::value &&
                std::is_unsigned<typename T3::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T3, typename T1::result_type>::
                              value>::value>::value,
        void>::type>
std::shuffle_order_engine<T1, T2> f14(T3 &a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
typename std::enable_if<
    std::integral_constant<
        bool, std::integral_constant<
                  bool, !std::is_same<
                            typename std::remove_cv<
                                typename std::remove_reference<T3>::type>::type,
                            std::shuffle_order_engine<T1, T2>>::value>::value &&
                  std::is_unsigned<typename T3::result_type>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<T3, typename T1::result_type>::
                                value>::value>::value,
    void>::type
f15(std::shuffle_order_engine<T1, T2> &a0, T3 &a1) {
  return a0.template seed<T3>(a1);
}
