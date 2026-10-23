#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <std::size_t T1, std::size_t T2>
using t3 = std::aligned_storage_t<T1, T2>;
