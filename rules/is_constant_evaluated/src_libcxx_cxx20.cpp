#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f1() { return std::is_constant_evaluated(); }
