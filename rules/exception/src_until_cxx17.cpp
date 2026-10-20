#include "../common/init.h"
#include "../common/traits.h"
#include <exception>
#include <new>
#include <utility>

using t4 = max_align_t;

using t5 = ptrdiff_t;

bool f14() { return std::uncaught_exception(); }
