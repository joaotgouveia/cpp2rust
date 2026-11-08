// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <type_traits>

int main() {
  assert(std::conjunction_v<>);
  assert(std::conjunction_v<std::true_type>);
  assert(!std::conjunction_v<std::false_type>);
  assert((std::conjunction_v<std::true_type, std::true_type>));
  assert((!std::conjunction_v<std::true_type, std::false_type>));

  assert(!std::disjunction_v<>);
  assert(std::disjunction_v<std::true_type>);
  assert(!std::disjunction_v<std::false_type>);
  assert((std::disjunction_v<std::false_type, std::true_type>));
  assert((!std::disjunction_v<std::false_type, std::false_type>));
  return 0;
}
