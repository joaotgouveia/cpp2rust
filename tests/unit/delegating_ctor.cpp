#include <cassert>

struct Foo {
  int a_;
  int b_;
  Foo(int a, int b) : a_(a), b_(b) {}
  Foo(int a, int *count) : Foo(a, 0) { *count += 1; }
  Foo() : Foo(7, 8) { a_ += 1; }
};

int main() {
  int count = 0;
  Foo f(3, &count);
  assert(f.a_ == 3);
  assert(f.b_ == 0);
  assert(count == 1);
  Foo g;
  assert(g.a_ == 8);
  assert(g.b_ == 8);
  return 0;
}
