#include <compare>
#include <thread>
#include <utility>

std::strong_ordering f8(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) <=> std::move(a1);
}
