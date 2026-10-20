#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1, typename... T2> using t1 = std::is_invocable<T1, T2...>;
