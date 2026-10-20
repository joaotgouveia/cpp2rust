#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t25 = std::ranges::fold_left_first_with_iter_result<T1, T2>;

template <typename T1, typename T2>
using t26 = std::ranges::fold_left_with_iter_result<T1, T2>;
