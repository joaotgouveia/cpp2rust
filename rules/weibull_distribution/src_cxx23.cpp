#include <random>
#include <utility>

template <typename T1> using t1 = std::weibull_distribution<T1>;

template <typename T1>
using t2 = typename std::weibull_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::weibull_distribution<T1>::param_type;

template <typename T1> std::weibull_distribution<T1> f1() {
  return std::weibull_distribution<T1>();
}

template <typename T1> std::weibull_distribution<T1> f2(T1 a0, T1 a1) {
  return std::weibull_distribution<T1>(std::move(a0), std::move(a1));
}

template <typename T1>
std::weibull_distribution<T1>
f3(const typename std::weibull_distribution<T1>::param_type &a0) {
  return std::weibull_distribution<T1>(a0);
}

template <typename T1> void f4(std::weibull_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1> T1 f5(const std::weibull_distribution<T1> &a0) {
  return a0.a();
}

template <typename T1> T1 f6(const std::weibull_distribution<T1> &a0) {
  return a0.b();
}

template <typename T1>
typename std::weibull_distribution<T1>::param_type
f7(const std::weibull_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f8(std::weibull_distribution<T1> &a0,
        const typename std::weibull_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1> T1 f9(const std::weibull_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1> T1 f10(const std::weibull_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
T1 f11(std::weibull_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
T1 f12(std::weibull_distribution<T1> &a0, T2 &a1,
       const typename std::weibull_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f13(const std::weibull_distribution<T1> &a0,
         const std::weibull_distribution<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1>
using t4 =
    typename std::weibull_distribution<T1>::param_type::distribution_type;

template <typename T1>
typename std::weibull_distribution<T1>::param_type f14() {
  return typename std::weibull_distribution<T1>::param_type();
}

template <typename T1>
typename std::weibull_distribution<T1>::param_type f15(T1 a0, T1 a1) {
  return typename std::weibull_distribution<T1>::param_type(std::move(a0),
                                                            std::move(a1));
}

template <typename T1>
T1 f16(const typename std::weibull_distribution<T1>::param_type &a0) {
  return a0.a();
}

template <typename T1>
T1 f17(const typename std::weibull_distribution<T1>::param_type &a0) {
  return a0.b();
}

template <typename T1>
bool f18(const typename std::weibull_distribution<T1>::param_type &a0,
         const typename std::weibull_distribution<T1>::param_type &a1) {
  return operator==(a0, a1);
}
