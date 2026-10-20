#include "../common/init.h"
#include "../common/traits.h"
#include <exception>
#include <new>
#include <utility>

int f15() { return std::uncaught_exceptions(); }
