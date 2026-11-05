#include <type_traits>
#include <utility>

using t1 = std::disjunction<>;

std::disjunction<> f1() { return std::disjunction<>(); }

std::disjunction<> f2(const std::disjunction<> &a0) {
  return std::disjunction<>(a0);
}

std::disjunction<> f3(std::disjunction<> &&a0) {
  return std::disjunction<>(std::move(a0));
}

std::disjunction<> &f4(std::disjunction<> &a0, const std::disjunction<> &a1) {
  return a0.operator=(a1);
}

std::disjunction<> &f5(std::disjunction<> &a0, std::disjunction<> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, false>::value_type
f6(const std::disjunction<> &a0) {
  return static_cast<std::integral_constant<bool, false>::value_type>(a0);
}

std::integral_constant<bool, false>::value_type
f7(const std::disjunction<> &a0) {
  return a0.operator()();
}

bool f8() { return std::disjunction<>::value; }

template <typename... T1> using t2 = std::disjunction<T1...>;
