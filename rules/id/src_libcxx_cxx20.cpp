#include <compare>
#include <thread>
#include <utility>

std::strong_ordering f13(std::__thread_id a0, std::__thread_id a1) {
  return std::operator<=>(std::move(a0), std::move(a1));
}
