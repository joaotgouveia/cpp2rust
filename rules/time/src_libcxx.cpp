#include <iomanip>
#include <utility>

using t1 = std::time_base;

using t2 = std::time_base::dateorder;

std::time_base f1() { return std::time_base(); }

std::time_base f2(const std::time_base &a0) { return std::time_base(a0); }

std::time_base f3(std::time_base &&a0) { return std::time_base(std::move(a0)); }

std::time_base &f4(std::time_base &a0, const std::time_base &a1) {
  return a0.operator=(a1);
}

std::time_base &f5(std::time_base &a0, std::time_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::time_base::dateorder f6() { return std::time_base::no_order; }

std::time_base::dateorder f7() { return std::time_base::dmy; }

std::time_base::dateorder f8() { return std::time_base::mdy; }

std::time_base::dateorder f9() { return std::time_base::ymd; }

std::time_base::dateorder f10() { return std::time_base::ydm; }
