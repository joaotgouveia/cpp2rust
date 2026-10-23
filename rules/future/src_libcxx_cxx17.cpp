#include <chrono>
#include <future>
#include <system_error>
#include <utility>

std::future_error f42(std::future_errc a0) { return std::future_error(a0); }
