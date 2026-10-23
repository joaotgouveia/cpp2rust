#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::chrono::treat_as_floating_point<T1>;
