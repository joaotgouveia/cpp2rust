#include <random>
#include <utility>

template <typename T1, T1 T2, T1 T3, T1 T4>
using t1 = std::linear_congruential_engine<T1, T2, T3, T4>;

template <typename T1, T1 T2, T1 T3, T1 T4>
using t2 =
    typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type;

template <typename T1, T1 T2, T1 T3, T1 T4>
std::linear_congruential_engine<T1, T2, T3, T4> f1() {
  return std::linear_congruential_engine<T1, T2, T3, T4>();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
std::linear_congruential_engine<T1, T2, T3, T4> f2(T1 a0) {
  return std::linear_congruential_engine<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, T1 T2, T1 T3, T1 T4>
void f3(std::linear_congruential_engine<T1, T2, T3, T4> &a0, T1 a1) {
  return a0.seed(std::move(a1));
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f4() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::min();
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f5() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::max();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
void f6(std::linear_congruential_engine<T1, T2, T3, T4> &a0,
        unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
T1 f7(std::linear_congruential_engine<T1, T2, T3, T4> &a0) {
  return a0.operator()();
}

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
std::linear_congruential_engine<T1, T2, T3, T4> f8(T5 &a0) {
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
f9(std::linear_congruential_engine<T1, T2, T3, T4> &a0, T5 &a1) {
  return a0.template seed<T5>(a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
bool f10(const std::linear_congruential_engine<T1, T2, T3, T4> &a0,
         const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f11() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::multiplier;
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f12() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::increment;
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f13() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::modulus;
}

template <typename T1, T1 T2, T1 T3, T1 T4> T1 f14() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::default_seed;
}
