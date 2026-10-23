#include <limits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t2 = std::float_denorm_style;

std::float_denorm_style f6() { return std::denorm_indeterminate; }

std::float_denorm_style f7() { return std::denorm_absent; }

std::float_denorm_style f8() { return std::denorm_present; }
