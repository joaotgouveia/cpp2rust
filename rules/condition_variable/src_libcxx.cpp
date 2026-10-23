#include <atomic>
#include <condition_variable>
#include <utility>

std::__libcpp_condvar_t *f28(std::condition_variable &a0) {
  return a0.native_handle();
}
