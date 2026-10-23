#include <chrono>
#include <future>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::future_error f41(std::future_errc a0) { return std::future_error(a0); }
