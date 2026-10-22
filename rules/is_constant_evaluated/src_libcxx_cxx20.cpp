#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

bool f1() { return std::is_constant_evaluated(); }
