#include <bit>
#include <utility>

template <typename T1> T1 f16(T1 a0) { return std::byteswap<T1>(a0); }
