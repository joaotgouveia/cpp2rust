#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::iterator_t<T1>;
