#include <coroutine>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2>
using t1 = std::coroutine_traits<T1, T2...>;
