#include <algorithm>
#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, std::size_t T3, typename T4,
    typename T5 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool,
                !std::is_same<
                    typename std::remove_cv<
                        typename std::remove_reference<T4>::type>::type,
                    std::discard_block_engine<T1, T2, T3>>::value>::value &&
                std::is_unsigned<typename T4::result_type>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T4, typename T1::result_type>::
                              value>::value>::value,
        void>::type>
std::discard_block_engine<T1, T2, T3> f14(T4 &a0) {
  return std::discard_block_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4>
typename std::enable_if<
    std::integral_constant<
        bool,
        std::integral_constant<
            bool, !std::is_same<
                      typename std::remove_cv<
                          typename std::remove_reference<T4>::type>::type,
                      std::discard_block_engine<T1, T2, T3>>::value>::value &&
            std::is_unsigned<typename T4::result_type>::value &&
            std::integral_constant<
                bool, !std::is_convertible<
                          T4, typename T1::result_type>::value>::value>::value,
    void>::type
f15(std::discard_block_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f16(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
