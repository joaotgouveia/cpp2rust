#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t11 = typename std::ostream_iterator<T1, T2, T3>::difference_type;
