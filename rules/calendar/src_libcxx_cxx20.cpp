#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

using t1 = std::chrono::last_spec;

std::chrono::last_spec f1() { return std::chrono::last_spec(); }

std::chrono::last_spec f2(const std::chrono::last_spec &a0) {
  return std::chrono::last_spec(a0);
}

std::chrono::last_spec f3(std::chrono::last_spec &&a0) {
  return std::chrono::last_spec(std::move(a0));
}

std::chrono::last_spec &f4(std::chrono::last_spec &a0,
                           const std::chrono::last_spec &a1) {
  return a0.operator=(a1);
}

std::chrono::last_spec &f5(std::chrono::last_spec &a0,
                           std::chrono::last_spec &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::local_days;

using t3 = std::chrono::local_seconds;

using t4 = std::chrono::local_t;

std::chrono::local_t f6() { return std::chrono::local_t(); }

std::chrono::local_t f7(const std::chrono::local_t &a0) {
  return std::chrono::local_t(a0);
}

std::chrono::local_t f8(std::chrono::local_t &&a0) {
  return std::chrono::local_t(std::move(a0));
}

std::chrono::local_t &f9(std::chrono::local_t &a0,
                         const std::chrono::local_t &a1) {
  return a0.operator=(a1);
}

std::chrono::local_t &f10(std::chrono::local_t &a0, std::chrono::local_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t5 = std::chrono::local_time<T1>;
