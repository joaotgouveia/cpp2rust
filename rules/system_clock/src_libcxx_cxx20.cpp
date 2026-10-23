#include <chrono>
#include <ratio>
#include <utility>

using t13 = std::chrono::sys_days;

using t14 = std::chrono::sys_seconds;

template <typename T1> using t15 = std::chrono::sys_time<T1>;

std::chrono::system_clock::time_point &
f36(std::chrono::system_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::system_clock::time_point
f37(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::time_point &
f38(std::chrono::system_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::system_clock::time_point
f39(std::chrono::system_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}
