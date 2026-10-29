#include <ios>
#include <utility>

using t1 = std::ctype_base;

using t2 = std::ctype_base::mask;

std::ctype_base f1() { return std::ctype_base(); }

std::ctype_base f2(const std::ctype_base &a0) { return std::ctype_base(a0); }

std::ctype_base f3(std::ctype_base &&a0) {
  return std::ctype_base(std::move(a0));
}

std::ctype_base &f4(std::ctype_base &a0, const std::ctype_base &a1) {
  return a0.operator=(a1);
}

std::ctype_base &f5(std::ctype_base &a0, std::ctype_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::ctype_base::mask f6() { return std::ctype_base::upper; }

std::ctype_base::mask f7() { return std::ctype_base::lower; }

std::ctype_base::mask f8() { return std::ctype_base::alpha; }

std::ctype_base::mask f9() { return std::ctype_base::digit; }

std::ctype_base::mask f10() { return std::ctype_base::xdigit; }

std::ctype_base::mask f11() { return std::ctype_base::space; }

std::ctype_base::mask f12() { return std::ctype_base::print; }

std::ctype_base::mask f13() { return std::ctype_base::graph; }

std::ctype_base::mask f14() { return std::ctype_base::cntrl; }

std::ctype_base::mask f15() { return std::ctype_base::punct; }

std::ctype_base::mask f16() { return std::ctype_base::alnum; }

std::ctype_base::mask f17() { return std::ctype_base::blank; }
