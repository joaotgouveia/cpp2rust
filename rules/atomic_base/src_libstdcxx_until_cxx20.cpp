#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <utility>

std::memory_order f8() { return std::memory_order_relaxed; }

std::memory_order f9() { return std::memory_order_consume; }

std::memory_order f10() { return std::memory_order_acquire; }

std::memory_order f11() { return std::memory_order_release; }

std::memory_order f12() { return std::memory_order_acq_rel; }

std::memory_order f13() { return std::memory_order_seq_cst; }
