#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
using t12 = typename std::istreambuf_iterator<T1, T2>::pointer;
