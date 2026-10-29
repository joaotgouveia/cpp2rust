#include <type_traits>
#include <utility>

template <typename T1, unsigned int T2> using t1 = std::extent<T1[], T2>;

template <typename T1, unsigned int T2, std::size_t T3>
using t2 = std::extent<T1[T3], T2>;

template <typename T1, unsigned int T2> using t3 = std::extent<T1, T2>;

template <typename T1, unsigned int T2>
unsigned long f1(const std::extent<T1, T2> &a0) {
  return static_cast<unsigned long>(a0);
}

template <typename T1, unsigned int T2>
unsigned long f2(const std::extent<T1, T2> &a0) {
  return a0.operator()();
}
