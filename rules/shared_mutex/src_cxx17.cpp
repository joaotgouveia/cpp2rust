#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <shared_mutex>
#include <utility>

using t2 = std::shared_mutex;

using t3 = std::shared_mutex::native_handle_type;

std::shared_mutex f17() { return {}; }

void f18(std::shared_mutex &a0) { return a0.lock(); }

bool f19(std::shared_mutex &a0) { return a0.try_lock(); }

void f20(std::shared_mutex &a0) { return a0.unlock(); }

void f21(std::shared_mutex &a0) { return a0.lock_shared(); }

bool f22(std::shared_mutex &a0) { return a0.try_lock_shared(); }

void f23(std::shared_mutex &a0) { return a0.unlock_shared(); }

void *f24(std::shared_mutex &a0) { return a0.native_handle(); }
