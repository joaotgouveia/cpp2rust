#include <compare>
#include <utility>

bool f1(std::partial_ordering a0) { return std::is_eq(std::move(a0)); }

bool f2(std::partial_ordering a0) { return std::is_gt(std::move(a0)); }

bool f3(std::partial_ordering a0) { return std::is_gteq(std::move(a0)); }

bool f4(std::partial_ordering a0) { return std::is_lt(std::move(a0)); }

bool f5(std::partial_ordering a0) { return std::is_lteq(std::move(a0)); }

bool f6(std::partial_ordering a0) { return std::is_neq(std::move(a0)); }
