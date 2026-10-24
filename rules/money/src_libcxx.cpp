#include <iomanip>
#include <utility>

using t1 = std::money_base;

using t2 = std::money_base::part;

using t3 = std::money_base::pattern;

std::money_base f1() { return std::money_base(); }

std::money_base f2(const std::money_base &a0) { return std::money_base(a0); }

std::money_base f3(std::money_base &&a0) {
  return std::money_base(std::move(a0));
}

std::money_base &f4(std::money_base &a0, const std::money_base &a1) {
  return a0.operator=(a1);
}

std::money_base &f5(std::money_base &a0, std::money_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::money_base::pattern f6() { return std::money_base::pattern(); }

std::money_base::pattern f7(const std::money_base::pattern &a0) {
  return std::money_base::pattern(a0);
}

std::money_base::pattern f8(std::money_base::pattern &&a0) {
  return std::money_base::pattern(std::move(a0));
}

std::money_base::pattern &f9(std::money_base::pattern &a0,
                             const std::money_base::pattern &a1) {
  return a0.operator=(a1);
}

std::money_base::pattern &f10(std::money_base::pattern &a0,
                              std::money_base::pattern &&a1) {
  return a0.operator=(std::move(a1));
}
