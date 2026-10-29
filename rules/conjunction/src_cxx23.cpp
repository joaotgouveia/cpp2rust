#include <type_traits>
#include <utility>

using t1 = std::conjunction<>;

std::conjunction<> f1() { return std::conjunction<>(); }

std::conjunction<> f2(const std::conjunction<> &a0) {
  return std::conjunction<>(a0);
}

std::conjunction<> f3(std::conjunction<> &&a0) {
  return std::conjunction<>(std::move(a0));
}

std::conjunction<> &f4(std::conjunction<> &a0, const std::conjunction<> &a1) {
  return a0.operator=(a1);
}

std::conjunction<> &f5(std::conjunction<> &a0, std::conjunction<> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f6(const std::conjunction<> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f7(const std::conjunction<> &a0) {
  return a0.operator()();
}

template <typename... T1> using t2 = std::conjunction<T1...>;
