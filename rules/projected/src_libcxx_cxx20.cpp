#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::projected<T1, T2>;
