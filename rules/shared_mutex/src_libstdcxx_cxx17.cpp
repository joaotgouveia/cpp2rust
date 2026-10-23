#include <chrono>
#include <shared_mutex>
#include <utility>

using t3 = std::shared_mutex::native_handle_type;

void *f24(std::shared_mutex &a0) { return a0.native_handle(); }
