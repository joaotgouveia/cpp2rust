#include <limits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t3 = std::float_denorm_style;

std::float_denorm_style f9() { return std::denorm_indeterminate; }

std::float_denorm_style f10() { return std::denorm_absent; }

std::float_denorm_style f11() { return std::denorm_present; }
