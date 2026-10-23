#include <chrono>
#include <utility>

using t1 = std::chrono::steady_clock;

using t2 = std::chrono::steady_clock::duration;

using t3 = std::chrono::steady_clock::rep;

using t4 = std::chrono::steady_clock::period;

using t5 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f1() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f2() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f3(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f4(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f5(std::chrono::steady_clock &a0,
                              const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f6(std::chrono::steady_clock &a0,
                              std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f7() { return std::chrono::steady_clock::is_steady; }
