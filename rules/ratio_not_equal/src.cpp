#include "../common/init.h"
#include "../common/traits.h"
#include <ratio>
#include <utility>

template <typename T1, typename T2> using t1 = std::ratio_not_equal<T1, T2>;
