#include <cassert>

template <typename T> struct Traits {
  enum Kind { Small, Large };
  enum { Size = sizeof(T) };
  static const int Width = sizeof(T) * 8;
  struct Box {
    T value;
  };

  static Kind kind() { return sizeof(T) > 2 ? Large : Small; }
  Box box(T v) { return Box{v}; }
};

template <typename T, typename U> struct IsSame {
  enum { value = 0 };
};
template <typename T> struct IsSame<T, T> {
  enum { value = 1 };
};

int main() {
  assert(Traits<char>::Size == 1);
  assert(Traits<int>::Size == 4);
  assert(Traits<char>::Width == 8);
  assert(Traits<int>::Width == 32);
  assert(Traits<char>::kind() == Traits<char>::Small);
  assert(Traits<int>::kind() == Traits<int>::Large);

  Traits<char> tc;
  Traits<int> ti;
  Traits<char>::Box bc = tc.box('a');
  Traits<int>::Box bi = ti.box(7);
  assert(bc.value == 'a');
  assert(bi.value == 7);

  assert((IsSame<int, int>::value == 1));
  assert((IsSame<int, char>::value == 0));
  assert((IsSame<char, char>::value == 1));
  return 0;
}
