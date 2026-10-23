#include <bit>
#include <utility>

template <typename T1> int f1(T1 a0) { return std::countr_one<T1>(a0); }

template <typename T1> int f2(T1 a0) { return std::countr_zero<T1>(a0); }
