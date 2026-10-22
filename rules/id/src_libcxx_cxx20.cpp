#include "../common/init.h"
#include "../common/traits.h"
#include <thread>
#include <utility>

auto f8() { return std::this_thread::get_id(); }
