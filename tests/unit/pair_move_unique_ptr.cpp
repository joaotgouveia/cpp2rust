#include <cassert>
#include <memory>
#include <utility>

int main() {
  std::unique_ptr<int> u(new int(3));
  std::pair<std::unique_ptr<int>, int> p(std::move(u), 1);
  assert(u.get() == nullptr);
  assert(*p.first == 3);
  std::pair<int, std::unique_ptr<int>> source(0, std::unique_ptr<int>(new int(81)));
  int *original = source.second.get();
  std::pair<long, std::unique_ptr<int>> converted(std::move(source));
  assert(converted.first == 0L);
  assert(converted.second.get() == original);
  assert(*converted.second == 81);
  assert(source.second.get() == nullptr);
  return 0;
}
