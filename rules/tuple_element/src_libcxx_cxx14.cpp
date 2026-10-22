#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <std::size_t T1, typename... T2>
using t24 = std::tuple_element_t<T1, T2...>;
