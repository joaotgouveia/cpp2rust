#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <utility>

std::memory_order f7() { return std::memory_order::relaxed; }

std::memory_order f8() { return std::memory_order::consume; }

std::memory_order f9() { return std::memory_order::acquire; }

std::memory_order f10() { return std::memory_order::release; }

std::memory_order f11() { return std::memory_order::acq_rel; }

std::memory_order f12() { return std::memory_order::seq_cst; }
