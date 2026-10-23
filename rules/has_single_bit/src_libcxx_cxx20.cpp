#include <bit>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> bool f1(T1 a0) { return std::has_single_bit<T1>(a0); }
