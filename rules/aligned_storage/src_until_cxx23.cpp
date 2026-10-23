#include <type_traits>
#include <utility>

template <std::size_t T1, std::size_t T2>
using t1 = std::aligned_storage<T1, T2>;

template <std::size_t T1, std::size_t T2>
using t2 = typename std::aligned_storage<T1, T2>::type;
