#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1> using t1 = std::chrono::is_clock<T1>;
