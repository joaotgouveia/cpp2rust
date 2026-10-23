#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> T1 *f1(T1 *a0) { return std::launder<T1>(a0); }
