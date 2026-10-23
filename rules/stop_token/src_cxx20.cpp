#include <stop_token>
#include <utility>

using t1 = std::stop_token;

std::stop_token f1() { return std::stop_token(); }

std::stop_token f2(const std::stop_token &a0) { return std::stop_token(a0); }

std::stop_token f3(std::stop_token &&a0) {
  return std::stop_token(std::move(a0));
}

std::stop_token &f4(std::stop_token &a0, const std::stop_token &a1) {
  return a0.operator=(a1);
}

std::stop_token &f5(std::stop_token &a0, std::stop_token &&a1) {
  return a0.operator=(std::move(a1));
}

bool f6(const std::stop_token &a0) { return a0.stop_possible(); }

bool f7(const std::stop_token &a0) { return a0.stop_requested(); }

void f8(std::stop_token &a0, std::stop_token &a1) { return a0.swap(a1); }

void f9(std::stop_token &a0, std::stop_token &a1) { return swap(a0, a1); }

bool f10(const std::stop_token &a0, const std::stop_token &a1) {
  return operator==(a0, a1);
}
