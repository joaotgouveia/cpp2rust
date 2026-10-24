#include <chrono>
#include <utility>

using t1 = std::chrono::file_clock;

template <typename T1> using t2 = std::chrono::file_time<T1>;

using t3 = std::filesystem::_FilesystemClock;

using t4 = std::filesystem::_FilesystemClock::rep;

using t5 = std::filesystem::_FilesystemClock::period;

using t6 = std::filesystem::_FilesystemClock::duration;

using t7 = std::filesystem::_FilesystemClock::time_point;

std::filesystem::_FilesystemClock::time_point f1() {
  return std::filesystem::_FilesystemClock::now();
}

std::filesystem::_FilesystemClock f2() {
  return std::filesystem::_FilesystemClock();
}

std::filesystem::_FilesystemClock
f3(const std::filesystem::_FilesystemClock &a0) {
  return std::filesystem::_FilesystemClock(a0);
}

std::filesystem::_FilesystemClock f4(std::filesystem::_FilesystemClock &&a0) {
  return std::filesystem::_FilesystemClock(std::move(a0));
}

std::filesystem::_FilesystemClock &
f5(std::filesystem::_FilesystemClock &a0,
   const std::filesystem::_FilesystemClock &a1) {
  return a0.operator=(a1);
}

std::filesystem::_FilesystemClock &f6(std::filesystem::_FilesystemClock &a0,
                                      std::filesystem::_FilesystemClock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<T1> f7(const std::chrono::file_time<T1> &a0) {
  return std::filesystem::_FilesystemClock::to_sys(a0);
}

template <typename T1>
std::chrono::file_time<T1> f8(const std::chrono::sys_time<T1> &a0) {
  return std::filesystem::_FilesystemClock::from_sys(a0);
}

bool f9() { return std::filesystem::_FilesystemClock::is_steady; }
