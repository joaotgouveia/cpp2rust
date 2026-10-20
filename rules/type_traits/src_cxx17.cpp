#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <bool T1> using t26 = std::bool_constant<T1>;

template <typename T1, typename... T2>
using t27 = std::invoke_result_t<T1, T2...>;

template <typename... T1> using t28 = std::void_t<T1...>;
