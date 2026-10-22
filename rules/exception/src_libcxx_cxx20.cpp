#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <exception>
#include <utility>

using t4 = std::chrono::ambiguous_local_time;

std::chrono::ambiguous_local_time
f16(const std::chrono::ambiguous_local_time &a0) {
  return std::chrono::ambiguous_local_time(a0);
}

std::chrono::ambiguous_local_time &
f17(std::chrono::ambiguous_local_time &a0,
    const std::chrono::ambiguous_local_time &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::chrono::ambiguous_local_time f18(const std::chrono::local_time<T1> &a0,
                                      const std::chrono::local_info &a1) {
  return std::chrono::ambiguous_local_time(a0, a1);
}

const char *f19(const std::chrono::ambiguous_local_time &a0) {
  return a0.what();
}

using t5 = std::chrono::nonexistent_local_time;

std::chrono::nonexistent_local_time
f20(const std::chrono::nonexistent_local_time &a0) {
  return std::chrono::nonexistent_local_time(a0);
}

std::chrono::nonexistent_local_time &
f21(std::chrono::nonexistent_local_time &a0,
    const std::chrono::nonexistent_local_time &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::chrono::nonexistent_local_time f22(const std::chrono::local_time<T1> &a0,
                                        const std::chrono::local_info &a1) {
  return std::chrono::nonexistent_local_time(a0, a1);
}

const char *f23(const std::chrono::nonexistent_local_time &a0) {
  return a0.what();
}
