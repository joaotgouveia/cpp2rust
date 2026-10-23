#include <exception>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::exception_ptr f2() { return std::current_exception(); }

void f3(std::exception_ptr a0) { return std::rethrow_exception(std::move(a0)); }
