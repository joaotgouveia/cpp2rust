#include "../common/init.h"
#include "../common/traits.h"
#include <locale>
#include <string>
#include <utility>

using t1 = std::messages_base;

using t2 = std::messages_base::catalog;

std::messages_base f1() { return std::messages_base(); }

std::messages_base f2(const std::messages_base &a0) {
  return std::messages_base(a0);
}

std::messages_base f3(std::messages_base &&a0) {
  return std::messages_base(std::move(a0));
}

std::messages_base &f4(std::messages_base &a0, const std::messages_base &a1) {
  return a0.operator=(a1);
}

std::messages_base &f5(std::messages_base &a0, std::messages_base &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::money_base;

using t4 = std::money_base::part;

using t5 = std::money_base::pattern;

std::money_base f6() { return std::money_base(); }

std::money_base f7(const std::money_base &a0) { return std::money_base(a0); }

std::money_base f8(std::money_base &&a0) {
  return std::money_base(std::move(a0));
}

std::money_base &f9(std::money_base &a0, const std::money_base &a1) {
  return a0.operator=(a1);
}

std::money_base &f10(std::money_base &a0, std::money_base &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::time_base;

using t7 = std::time_base::dateorder;

std::time_base f11() { return std::time_base(); }

std::time_base f12(const std::time_base &a0) { return std::time_base(a0); }

std::time_base f13(std::time_base &&a0) {
  return std::time_base(std::move(a0));
}

std::time_base &f14(std::time_base &a0, const std::time_base &a1) {
  return a0.operator=(a1);
}

std::time_base &f15(std::time_base &a0, std::time_base &&a1) {
  return a0.operator=(std::move(a1));
}
