#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::fold_left_first_with_iter_result<T1, T2>;

template <typename T1, typename T2>
using t2 = std::ranges::fold_left_with_iter_result<T1, T2>;
