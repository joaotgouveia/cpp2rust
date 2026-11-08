// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <tuple>

int main() {
  std::tuple<int, double> two(1, 2.5);
  assert(std::get<0>(two) == 1);
  assert(std::get<1>(two) == 2.5);
  std::get<0>(two) = 7;
  assert(std::get<0>(two) == 7);

  auto made = std::make_tuple(1, 2.5, 'c');
  assert(std::get<2>(made) == 'c');
  assert(made == std::make_tuple(1, 2.5, 'c'));
  assert(!(made == std::make_tuple(2, 2.5, 'c')));
  assert(!(std::make_tuple(1, 2) == std::make_tuple(2, 1)));

  std::tuple<long> single(3);
  assert(std::get<0>(single) == 3);

  std::tuple<long, double> converted(std::tuple<int, float>(4, 1.5f));
  assert(std::get<0>(converted) == 4);
  assert(std::get<1>(converted) == 1.5);

  std::tuple<> empty;
  assert(empty == std::make_tuple());
  return 0;
}
