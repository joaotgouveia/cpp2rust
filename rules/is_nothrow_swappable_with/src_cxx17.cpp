#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t1 = std::is_nothrow_swappable_with<T1, T2>;
