#include "../common/init.h"
#include "../common/traits.h"
#include <new>
#include <utility>

std::new_handler f1() { return std::get_new_handler(); }

using t1 = std::new_handler;

std::new_handler f2(std::new_handler a0) { return std::set_new_handler(a0); }
