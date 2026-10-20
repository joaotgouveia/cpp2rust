#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::elements_of<T1, T2>;
