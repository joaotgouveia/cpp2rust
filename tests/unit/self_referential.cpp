#include <cassert>

struct SelfPtr {
  int value_{};
  SelfPtr *this_ = this;
  SelfPtr() = default;
};

struct Alloc {
  int id;
};

struct RefToMember {
  Alloc a_{7};
  const Alloc &alloc_ = a_;
  bool constructed_ = false;
  RefToMember() = default;
};

struct CtorSelfPtr {
  int value_;
  CtorSelfPtr *this_;
  CtorSelfPtr(int v) : value_(v), this_(this) {}
};

struct CtorReadsMember {
  int size;
  int doubled;
  CtorReadsMember(int s) : size(s), doubled(size * 2) {}
};

struct list_head {
  struct list_head *next;
};

static bool f() {
  struct list_head list = {&list};
  return list.next == &list;
}

int main() {
  SelfPtr s;
  assert(s.this_ == &s);
  assert(s.value_ == 0);

  RefToMember r;
  assert(&r.alloc_ == &r.a_);
  assert(r.alloc_.id == 7);
  assert(!r.constructed_);

  CtorSelfPtr c(3);
  assert(c.this_ == &c);
  assert(c.value_ == 3);

  CtorReadsMember m(4);
  assert(m.doubled == 8);

  assert(f());
  return 0;
}
