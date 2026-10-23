#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, typename T3, typename T4,
    typename T5 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool,
                !std::is_same<
                    typename std::remove_cv<
                        typename std::remove_reference<T4>::type>::type,
                    std::independent_bits_engine<T1, T2, T3>>::value>::value &&
                std::is_unsigned<typename T4::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T4, T3>::value>::value>::value,
        void>::type>
std::independent_bits_engine<T1, T2, T3> f13(T4 &a0) {
  return std::independent_bits_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
typename std::enable_if<
    std::integral_constant<
        bool, std::integral_constant<
                  bool, !std::is_same<
                            typename std::remove_cv<
                                typename std::remove_reference<T4>::type>::type,
                            std::independent_bits_engine<T1, T2, T3>>::value>::
                      value &&
                  std::is_unsigned<typename T4::result_type>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<T4, T3>::value>::value>::value,
    void>::type
f14(std::independent_bits_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}
