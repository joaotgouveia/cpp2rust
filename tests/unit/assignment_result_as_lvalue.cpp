#include <cassert>
#include <utility>

int main() {
  int a = 1;
  int b = 42;
  int &r = (a = std::move(b));
  assert(a == 42);
  assert(&r == &a);
  int arr[5] = {1, 2, 3, 4, 5};
  int *p = arr;
  int *&pr = (p += 3);
  assert(*pr == 4);
  assert(&pr == &p);
  (a = b) = 5;
  assert(a == 5);
  assert(b == 42);
  r = 7;
  assert(a == 7);
  int i = 0;
  int &q = (arr[i++] = 9);
  assert(i == 1);
  assert(&q == &arr[0]);
  assert(arr[0] == 9);
  int &s = (arr[i++] += 10);
  assert(i == 2);
  assert(&s == &arr[1]);
  assert(arr[1] == 12);
  int k = 3;
  int &t = (a = k++);
  assert(k == 4);
  assert(a == 3);
  assert(&t == &a);
  return 0;
}
