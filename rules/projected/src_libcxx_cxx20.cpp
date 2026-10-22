#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2> using t1 = std::projected<T1, T2>;
