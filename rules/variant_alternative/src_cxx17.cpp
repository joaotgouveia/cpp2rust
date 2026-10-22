#include "../common/init.h"
#include "../common/traits.h"
#include <tuple>
#include <utility>
#include <variant>

template <std::size_t T1, typename T2>
using t1 = std::variant_alternative<T1, const T2>;

template <std::size_t T1, typename T2>
using t2 = std::variant_alternative<T1, volatile T2>;

template <std::size_t T1, typename T2>
using t3 = std::variant_alternative<T1, const volatile T2>;

template <std::size_t T1, typename... T2>
using t4 = std::variant_alternative<T1, std::variant<T2...>>;

template <std::size_t T1, typename... T2>
using t5 = typename std::variant_alternative<T1, std::variant<T2...>>::type;
