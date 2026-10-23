#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::remove_copy_if_result<T1, T2>;
