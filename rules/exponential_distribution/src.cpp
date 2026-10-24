#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1> using t1 = std::exponential_distribution<T1>;

template <typename T1>
using t2 = typename std::exponential_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::exponential_distribution<T1>::param_type;

template <typename T1> std::exponential_distribution<T1> f1() {
  return std::exponential_distribution<T1>();
}

template <typename T1> std::exponential_distribution<T1> f2(T1 a0) {
  return std::exponential_distribution<T1>(std::move(a0));
}

template <typename T1>
std::exponential_distribution<T1>
f3(const typename std::exponential_distribution<T1>::param_type &a0) {
  return std::exponential_distribution<T1>(a0);
}

template <typename T1> void f4(std::exponential_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1> T1 f5(const std::exponential_distribution<T1> &a0) {
  return a0.lambda();
}

template <typename T1>
typename std::exponential_distribution<T1>::param_type
f6(const std::exponential_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f7(std::exponential_distribution<T1> &a0,
        const typename std::exponential_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1> T1 f8(const std::exponential_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1> T1 f9(const std::exponential_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
T1 f10(std::exponential_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
T1 f11(std::exponential_distribution<T1> &a0, T2 &a1,
       const typename std::exponential_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f12(const std::exponential_distribution<T1> &a0,
         const std::exponential_distribution<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1>
using t4 =
    typename std::exponential_distribution<T1>::param_type::distribution_type;

template <typename T1>
typename std::exponential_distribution<T1>::param_type f13(T1 a0) {
  return typename std::exponential_distribution<T1>::param_type(std::move(a0));
}

template <typename T1>
T1 f14(const typename std::exponential_distribution<T1>::param_type &a0) {
  return a0.lambda();
}

template <typename T1>
bool f15(const typename std::exponential_distribution<T1>::param_type &a0,
         const typename std::exponential_distribution<T1>::param_type &a1) {
  return operator==(a0, a1);
}
