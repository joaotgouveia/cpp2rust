#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::is_arithmetic<T1>;
