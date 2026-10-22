#include "../common/init.h"
#include "../common/traits.h"
#include <bit>
#include <utility>

template <typename T1> T1 f1(T1 a0) { return std::byteswap<T1>(a0); }
