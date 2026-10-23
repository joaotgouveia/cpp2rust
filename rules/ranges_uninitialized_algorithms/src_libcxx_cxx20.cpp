#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t1 = std::ranges::uninitialized_copy_n_result<T1, T2>;

template <typename T1, typename T2>
using t2 = std::ranges::uninitialized_copy_result<T1, T2>;

template <typename T1, typename T2>
using t3 = std::ranges::uninitialized_move_n_result<T1, T2>;

template <typename T1, typename T2>
using t4 = std::ranges::uninitialized_move_result<T1, T2>;
