#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t2 = std::ranges::keys_view<T1>;

template <typename T1> using t3 = std::ranges::values_view<T1>;
