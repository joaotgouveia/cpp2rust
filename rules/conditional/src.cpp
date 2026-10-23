#include <type_traits>
#include <utility>

template <bool T1, typename T2, typename T3>
using t1 = std::conditional<T1, T2, T3>;

template <bool T1, typename T2, typename T3>
using t2 = typename std::conditional<T1, T2, T3>::type;
