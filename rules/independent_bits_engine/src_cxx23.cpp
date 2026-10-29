#include <istream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3>
using t1 = std::independent_bits_engine<T1, T2, T3>;

template <typename T1, std::size_t T2, typename T3>
using t2 = typename std::independent_bits_engine<T1, T2, T3>::result_type;

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f1() {
  return std::independent_bits_engine<T1, T2, T3>();
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f2(const T1 &a0) {
  return std::independent_bits_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f3(T1 &&a0) {
  return std::independent_bits_engine<T1, T2, T3>(std::move(a0));
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f4(T3 a0) {
  return std::independent_bits_engine<T1, T2, T3>(std::move(a0));
}

template <typename T1, std::size_t T2, typename T3>
void f5(std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.seed();
}

template <typename T1, std::size_t T2, typename T3>
void f6(std::independent_bits_engine<T1, T2, T3> &a0, T3 a1) {
  return a0.seed(std::move(a1));
}

template <typename T1, std::size_t T2, typename T3>
const T1 &f7(const std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, typename T3> T3 f8() {
  return std::independent_bits_engine<T1, T2, T3>::min();
}

template <typename T1, std::size_t T2, typename T3> T3 f9() {
  return std::independent_bits_engine<T1, T2, T3>::max();
}

template <typename T1, std::size_t T2, typename T3>
void f10(std::independent_bits_engine<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, typename T3>
T3 f11(std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.operator()();
}

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
std::independent_bits_engine<T1, T2, T3> f12(T4 &a0)
  requires(!(std::is_same<T4, const T1>::value))
{
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
f13(std::independent_bits_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4, typename T5>
std::basic_istream<T4, T5> &f14(std::basic_istream<T4, T5> &a0,
                                std::independent_bits_engine<T1, T2, T3> &a1) {
  return operator>>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3>
bool f15(const std::independent_bits_engine<T1, T2, T3> &a0,
         const std::independent_bits_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
