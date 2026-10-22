#include "../common/init.h"
#include "../common/traits.h"
#include <exception>
#include <new>
#include <utility>

bool f14() { return std::uncaught_exception(); }
