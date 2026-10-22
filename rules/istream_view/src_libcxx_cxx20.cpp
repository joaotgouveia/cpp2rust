#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::istream_view<T1>;

template <typename T1> using t2 = std::ranges::wistream_view<T1>;
