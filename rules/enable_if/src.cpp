#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::enable_if<true, T1>;

template <typename T1> using t2 = typename std::enable_if<true, T1>::type;

template <bool T1, typename T2> using t3 = std::enable_if<T1, T2>;
