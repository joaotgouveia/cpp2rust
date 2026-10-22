#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::indirectly_readable_traits<T1>;

template <typename T1>
using t2 = typename std::indirectly_readable_traits<T1>::value_type;
