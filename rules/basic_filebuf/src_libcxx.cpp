#include <fstream>
#include <ios>
#include <string.h>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t7 = typename std::basic_filebuf<T1, T2>::state_type;
