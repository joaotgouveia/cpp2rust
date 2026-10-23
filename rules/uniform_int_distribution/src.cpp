#include <algorithm>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1> using t1 = std::uniform_int_distribution<T1>;

template <typename T1>
using t2 = typename std::uniform_int_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::uniform_int_distribution<T1>::param_type;

template <typename T1> std::uniform_int_distribution<T1> f1() {
  return std::uniform_int_distribution<T1>();
}

template <typename T1>
std::uniform_int_distribution<T1>
f2(const typename std::uniform_int_distribution<T1>::param_type &a0) {
  return std::uniform_int_distribution<T1>(a0);
}

template <typename T1> void f3(std::uniform_int_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1>
typename std::uniform_int_distribution<T1>::result_type
f4(const std::uniform_int_distribution<T1> &a0) {
  return a0.a();
}

template <typename T1>
typename std::uniform_int_distribution<T1>::result_type
f5(const std::uniform_int_distribution<T1> &a0) {
  return a0.b();
}

template <typename T1>
typename std::uniform_int_distribution<T1>::param_type
f6(const std::uniform_int_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f7(std::uniform_int_distribution<T1> &a0,
        const typename std::uniform_int_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1>
typename std::uniform_int_distribution<T1>::result_type
f8(const std::uniform_int_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1>
typename std::uniform_int_distribution<T1>::result_type
f9(const std::uniform_int_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
typename std::uniform_int_distribution<T1>::result_type
f10(std::uniform_int_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
typename std::uniform_int_distribution<T1>::result_type
f11(std::uniform_int_distribution<T1> &a0, T2 &a1,
    const typename std::uniform_int_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f12(const std::uniform_int_distribution<T1> &a0,
         const std::uniform_int_distribution<T1> &a1) {
  return operator==(a0, a1);
}
