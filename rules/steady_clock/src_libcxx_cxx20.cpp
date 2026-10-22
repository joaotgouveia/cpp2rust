#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <ratio>
#include <utility>

std::chrono::steady_clock::time_point &
f34(std::chrono::steady_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::time_point
f35(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::time_point &
f36(std::chrono::steady_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::time_point
f37(std::chrono::steady_clock::time_point &a0, int a1) {
  return a0.operator--(a1);
}
