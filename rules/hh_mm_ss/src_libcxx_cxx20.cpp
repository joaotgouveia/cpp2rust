#include <chrono>
#include <utility>

bool f11(const std::chrono::hours &a0) { return std::chrono::is_am(a0); }

bool f12(const std::chrono::hours &a0) { return std::chrono::is_pm(a0); }

std::chrono::hours f13(const std::chrono::hours &a0) {
  return std::chrono::make12(a0);
}

std::chrono::hours f14(const std::chrono::hours &a0, bool a1) {
  return std::chrono::make24(a0, a1);
}
