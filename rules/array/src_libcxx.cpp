#include <array>
#include <utility>

template <typename T1, std::size_t T2>
using t27 = typename std::array<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t28 = typename std::array<T1, T2>::reverse_iterator::value_type;

template <typename T1, std::size_t T2>
using t29 =
    typename std::array<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, std::size_t T2>
using t30 = typename std::array<T1, T2>::const_reverse_iterator::value_type;
