#include <coroutine>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::noop_coroutine_handle f1() { return std::noop_coroutine(); }

using t1 = std::noop_coroutine_handle;

using t2 = std::noop_coroutine_promise;

std::noop_coroutine_promise f2() { return std::noop_coroutine_promise(); }

std::noop_coroutine_promise f3(const std::noop_coroutine_promise &a0) {
  return std::noop_coroutine_promise(a0);
}

std::noop_coroutine_promise f4(std::noop_coroutine_promise &&a0) {
  return std::noop_coroutine_promise(std::move(a0));
}

std::noop_coroutine_promise &f5(std::noop_coroutine_promise &a0,
                                const std::noop_coroutine_promise &a1) {
  return a0.operator=(a1);
}

std::noop_coroutine_promise &f6(std::noop_coroutine_promise &a0,
                                std::noop_coroutine_promise &&a1) {
  return a0.operator=(std::move(a1));
}
