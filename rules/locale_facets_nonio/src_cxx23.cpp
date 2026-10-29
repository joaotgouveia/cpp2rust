#include <ios>
#include <locale>
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

std::money_base::pattern f11() { return std::money_base::pattern(); }

std::money_base::pattern f12(const std::money_base::pattern &a0) {
  return std::money_base::pattern(a0);
}

std::money_base::pattern f13(std::money_base::pattern &&a0) {
  return std::money_base::pattern(std::move(a0));
}

std::money_base::pattern &f14(std::money_base::pattern &a0,
                              const std::money_base::pattern &a1) {
  return a0.operator=(a1);
}

std::money_base::pattern &f15(std::money_base::pattern &a0,
                              std::money_base::pattern &&a1) {
  return a0.operator=(std::move(a1));
}

std::money_base::part f16() { return std::money_base::none; }

std::money_base::part f17() { return std::money_base::space; }

std::money_base::part f18() { return std::money_base::symbol; }

std::money_base::part f19() { return std::money_base::sign; }

std::money_base::part f20() { return std::money_base::value; }

using t6 = std::time_base;

using t7 = std::time_base::dateorder;

std::time_base f21() { return std::time_base(); }

std::time_base f22(const std::time_base &a0) { return std::time_base(a0); }

std::time_base f23(std::time_base &&a0) {
  return std::time_base(std::move(a0));
}

std::time_base &f24(std::time_base &a0, const std::time_base &a1) {
  return a0.operator=(a1);
}

std::time_base &f25(std::time_base &a0, std::time_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::time_base::dateorder f26() { return std::time_base::no_order; }

std::time_base::dateorder f27() { return std::time_base::dmy; }

std::time_base::dateorder f28() { return std::time_base::mdy; }

std::time_base::dateorder f29() { return std::time_base::ymd; }

std::time_base::dateorder f30() { return std::time_base::ydm; }
