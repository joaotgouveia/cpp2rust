#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <utility>

std::memory_order f1() { return std::memory_order_relaxed; }

std::memory_order f2() { return std::memory_order_consume; }

std::memory_order f3() { return std::memory_order_acquire; }

std::memory_order f4() { return std::memory_order_release; }

std::memory_order f5() { return std::memory_order_acq_rel; }

std::memory_order f6() { return std::memory_order_seq_cst; }
