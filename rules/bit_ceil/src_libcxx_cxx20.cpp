#include <bit>
#include <utility>

template <typename T1> T1 f1(T1 a0) { return std::bit_ceil<T1>(a0); }
