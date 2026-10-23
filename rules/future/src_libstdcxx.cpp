#include <future>
#include <system_error>
#include <utility>

std::future_error f35(std::future_errc a0) { return std::future_error(a0); }
