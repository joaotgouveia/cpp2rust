#include <assert.h>
#include <cstddef>
#include <utility>

template <class T> struct S {
  static bool called;
  static std::pair<T *, std::size_t> last;

  void record(T *p, std::size_t n) {
    called = false;
    last = std::pair<T *, std::size_t>(p, n);
  }
};

template <class T> bool S<T>::called = true;
template <class T> std::pair<T *, std::size_t> S<T>::last;

int main() {
  int x = 0;
  S<int> t;
  assert(S<int>::called);
  t.record(&x, 3);
  assert(!S<int>::called);
  assert(S<int>::last.first == &x);
  assert(S<int>::last.second == 3);
  return 0;
}
