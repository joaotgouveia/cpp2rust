#include <utility>
#include <variant>

#include "../common/init.h"
#include "../common/traits.h"

template <std::size_t T1, typename T2>
using t6 = typename std::variant_alternative<T1, const T2>::type;

template <std::size_t T1, typename T2>
using t7 = typename std::variant_alternative<T1, volatile T2>::type;

template <std::size_t T1, typename T2>
using t8 = typename std::variant_alternative<T1, const volatile T2>::type;
