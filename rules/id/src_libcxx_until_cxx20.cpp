#include <thread>
#include <utility>

bool f1(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) != std::move(a1);
}

bool f2(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) < std::move(a1);
}

bool f3(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) <= std::move(a1);
}

bool f4(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) > std::move(a1);
}

bool f5(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) >= std::move(a1);
}
