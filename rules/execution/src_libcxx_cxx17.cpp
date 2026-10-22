#include "../common/init.h"
#include "../common/traits.h"
#include <execution>
#include <utility>

using t1 = std::execution::parallel_policy;

using t2 = std::execution::parallel_unsequenced_policy;

using t3 = std::execution::sequenced_policy;
