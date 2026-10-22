#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <bool T1, typename T2, typename T3>
using t3 = std::conditional_t<T1, T2, T3>;
