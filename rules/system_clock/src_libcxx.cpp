#include <chrono>
#include <utility>

using t1 = std::chrono::system_clock;

using t2 = std::chrono::system_clock::duration;

using t3 = std::chrono::system_clock::rep;

using t4 = std::chrono::system_clock::period;

using t5 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f1() {
  return std::chrono::system_clock::now();
}

std::time_t f2(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f3(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f4() { return std::chrono::system_clock(); }

std::chrono::system_clock f5(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f6(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f7(std::chrono::system_clock &a0,
                              const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f8(std::chrono::system_clock &a0,
                              std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f9() { return std::chrono::system_clock::is_steady; }
