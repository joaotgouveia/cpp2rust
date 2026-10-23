#include <cstdlib>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::ldiv_t f1(long a0, long a1) { return std::div(a0, a1); }
