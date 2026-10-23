#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2> using t1 = std::result_of<T1(T2...)>;
