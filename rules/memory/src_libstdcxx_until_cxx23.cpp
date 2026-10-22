#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <memory>
#include <utility>

void f1(char *a0, std::size_t a1) { return std::declare_no_pointers(a0, a1); }

void f2(void *a0) { return std::declare_reachable(a0); }

std::pointer_safety f3() { return std::get_pointer_safety(); }

using t1 = std::pointer_safety;

std::pointer_safety f4() { return std::pointer_safety::relaxed; }

std::pointer_safety f5() { return std::pointer_safety::preferred; }

std::pointer_safety f6() { return std::pointer_safety::strict; }

void f7(char *a0, std::size_t a1) { return std::undeclare_no_pointers(a0, a1); }

template <typename T1> T1 *f8(T1 *a0) {
  return std::undeclare_reachable<T1>(a0);
}
