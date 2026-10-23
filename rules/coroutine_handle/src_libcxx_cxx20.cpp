#include <compare>
#include <coroutine>
#include <cstddef>
#include <utility>

bool f40(std::coroutine_handle<> a0, std::coroutine_handle<> a1) {
  return std::move(a0) == std::move(a1);
}

std::strong_ordering f41(std::coroutine_handle<> a0,
                         std::coroutine_handle<> a1) {
  return std::move(a0) <=> std::move(a1);
}
