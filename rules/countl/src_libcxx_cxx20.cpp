#include <bit>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> int f1(T1 a0) { return std::countl_one<T1>(a0); }

template <typename T1> int f2(T1 a0) { return std::countl_zero<T1>(a0); }
