#include <array>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2>
using t23 = typename std::array<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t24 = typename std::array<T1, T2>::reverse_iterator::value_type;

template <typename T1, std::size_t T2>
using t25 =
    typename std::array<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t26 = typename std::array<T1, T2>::const_reverse_iterator::value_type;
