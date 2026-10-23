#include <exception>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

int f15() { return std::uncaught_exceptions(); }
