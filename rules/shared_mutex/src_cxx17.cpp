#include <chrono>
#include <shared_mutex>
#include <utility>

using t2 = std::shared_mutex;

std::shared_mutex f13() { return {}; }

void f14(std::shared_mutex &a0) { return a0.lock(); }

bool f15(std::shared_mutex &a0) { return a0.try_lock(); }

void f16(std::shared_mutex &a0) { return a0.unlock(); }

void f17(std::shared_mutex &a0) { return a0.lock_shared(); }

bool f18(std::shared_mutex &a0) { return a0.try_lock_shared(); }

void f19(std::shared_mutex &a0) { return a0.unlock_shared(); }
