#include <cstddef>
#include <initializer_list>
#include <random>
#include <utility>

using t1 = std::seed_seq;

using t2 = std::seed_seq::result_type;

std::seed_seq f1() { return {}; }

std::size_t f2(const std::seed_seq &a0) { return a0.size(); }

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
std::seed_seq f3(std::initializer_list<T1> a0) {
  return {std::forward<std::initializer_list<T1>>(a0)};
}

template <typename T1> std::seed_seq f4(T1 a0, T1 a1) {
  return {std::move(a0), std::move(a1)};
}

template <typename T1> void f5(std::seed_seq &a0, T1 a1, T1 a2) {
  return a0.template generate<T1>(std::move(a1), std::move(a2));
}

template <typename T1> void f6(const std::seed_seq &a0, T1 a1) {
  return a0.template param<T1>(std::move(a1));
}
