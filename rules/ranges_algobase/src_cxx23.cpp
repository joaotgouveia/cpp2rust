#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::copy_backward_result<T1, T2>;

template <typename T1, typename T2>
using t2 = std::ranges::copy_n_result<T1, T2>;

template <typename T1, typename T2> using t3 = std::ranges::copy_result<T1, T2>;

template <typename T1, typename T2>
using t4 = std::ranges::move_backward_result<T1, T2>;

template <typename T1, typename T2> using t5 = std::ranges::move_result<T1, T2>;
