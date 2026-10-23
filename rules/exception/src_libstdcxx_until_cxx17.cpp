#include <exception>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f14() { return std::uncaught_exception(); }
