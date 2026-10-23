#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename... T3>
using t1 = std::is_nothrow_invocable_r<T1, T2, T3...>;
