#include <compare>
#include <thread>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::strong_ordering f8(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) <=> std::move(a1);
}
