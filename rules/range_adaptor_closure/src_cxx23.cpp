#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::range_adaptor_closure<T1>;
