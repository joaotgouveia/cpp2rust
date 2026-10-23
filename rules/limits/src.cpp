#include <limits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::float_round_style;

std::float_round_style f1() { return std::round_indeterminate; }

std::float_round_style f2() { return std::round_toward_zero; }

std::float_round_style f3() { return std::round_to_nearest; }

std::float_round_style f4() { return std::round_toward_infinity; }

std::float_round_style f5() { return std::round_toward_neg_infinity; }
