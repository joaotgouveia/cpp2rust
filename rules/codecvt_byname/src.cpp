#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <codecvt>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::codecvt_byname<T1, T2, T3>;
