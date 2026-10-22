#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <utility>

template <std::size_t... T1> using t3 = std::index_sequence<T1...>;

template <typename... T1> using t4 = std::index_sequence_for<T1...>;

template <std::size_t T1> using t5 = std::make_index_sequence<T1>;

template <typename T1, T1 T2> using t6 = std::make_integer_sequence<T1, T2>;
