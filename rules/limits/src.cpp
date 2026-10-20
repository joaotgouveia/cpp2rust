#include "../common/init.h"
#include "../common/traits.h"
#include <limits>
#include <utility>

using t1 = std::float_denorm_style;

std::float_denorm_style f1() { return std::denorm_indeterminate; }

std::float_denorm_style f2() { return std::denorm_absent; }

std::float_denorm_style f3() { return std::denorm_present; }

using t2 = std::float_round_style;

std::float_round_style f4() { return std::round_indeterminate; }

std::float_round_style f5() { return std::round_toward_zero; }

std::float_round_style f6() { return std::round_to_nearest; }

std::float_round_style f7() { return std::round_toward_infinity; }

std::float_round_style f8() { return std::round_toward_neg_infinity; }
