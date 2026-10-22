#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <ostream>
#include <thread>
#include <utility>

bool f2(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) != std::move(a1);
}

bool f3(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) < std::move(a1);
}

bool f4(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) <= std::move(a1);
}

bool f5(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) > std::move(a1);
}

bool f6(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) >= std::move(a1);
}
