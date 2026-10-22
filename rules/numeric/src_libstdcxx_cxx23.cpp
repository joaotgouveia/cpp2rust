#include "../common/init.h"
#include "../common/traits.h"
#include <numeric>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::iota_result<T1, T2>;
