#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::chrono::file_clock;

template <typename T1> using t2 = std::chrono::file_time<T1>;
