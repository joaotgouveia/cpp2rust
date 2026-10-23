#include <thread>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

bool f6(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) == std::move(a1);
}

std::thread::id f7() { return std::this_thread::get_id(); }
