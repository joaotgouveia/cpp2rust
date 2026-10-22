#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1>
using t3 = std::tuple_size<const std::__enable_if_has_tuple_size<T1>>;

template <typename T1>
using t4 = std::tuple_size<volatile std::__enable_if_has_tuple_size<T1>>;

template <typename T1>
using t5 = std::tuple_size<const volatile std::__enable_if_has_tuple_size<T1>>;
