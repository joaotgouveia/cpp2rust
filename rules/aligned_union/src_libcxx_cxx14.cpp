#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <type_traits>
#include <utility>

template <std::size_t T1, typename... T2>
using t5 = std::aligned_union_t<T1, T2...>;
