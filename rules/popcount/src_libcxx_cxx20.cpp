#include <bit>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> int f1(T1 a0) { return std::popcount<T1>(a0); }
