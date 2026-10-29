#include <algorithm>
#include <utility>
#include <vector>

using t1 = std::_Bit_const_iterator;

using t2 = std::_Bit_const_iterator::reference;

using t3 = std::_Bit_const_iterator::const_reference;

using t4 = std::_Bit_const_iterator::pointer;

using t5 = std::_Bit_const_iterator::const_iterator;

std::_Bit_const_iterator f1() { return std::_Bit_const_iterator(); }

std::_Bit_const_iterator f2(unsigned long *a0, unsigned int a1) {
  return std::_Bit_const_iterator(a0, a1);
}

std::_Bit_const_iterator f3(const std::_Bit_iterator &a0) {
  return std::_Bit_const_iterator(a0);
}

std::_Bit_const_iterator::const_reference
f4(const std::_Bit_const_iterator &a0) {
  return a0.operator*();
}

std::_Bit_const_iterator::const_iterator &f5(std::_Bit_const_iterator &a0) {
  return a0.operator++();
}

std::_Bit_const_iterator::const_iterator f6(std::_Bit_const_iterator &a0,
                                            int a1) {
  return a0.operator++(a1);
}

std::_Bit_const_iterator::const_iterator &f7(std::_Bit_const_iterator &a0) {
  return a0.operator--();
}

std::_Bit_const_iterator::const_iterator f8(std::_Bit_const_iterator &a0,
                                            int a1) {
  return a0.operator--(a1);
}

std::_Bit_const_iterator::const_iterator &
f9(std::_Bit_const_iterator &a0,
   std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator+=(a1);
}

std::_Bit_const_iterator::const_iterator &
f10(std::_Bit_const_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator-=(a1);
}

std::_Bit_const_iterator::const_reference
f11(const std::_Bit_const_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator[](a1);
}

std::_Bit_const_iterator f12(const std::_Bit_const_iterator &a0) {
  return std::_Bit_const_iterator(a0);
}

std::_Bit_const_iterator f13(std::_Bit_const_iterator &&a0) {
  return std::_Bit_const_iterator(std::move(a0));
}

std::_Bit_const_iterator &f14(std::_Bit_const_iterator &a0,
                              const std::_Bit_const_iterator &a1) {
  return a0.operator=(a1);
}

std::_Bit_const_iterator &f15(std::_Bit_const_iterator &a0,
                              std::_Bit_const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::_Bit_const_iterator::const_iterator
f16(std::iterator<std::random_access_iterator_tag, bool>::difference_type a0,
    const std::_Bit_const_iterator::const_iterator &a1) {
  return operator+(a0, a1);
}

std::_Bit_const_iterator::const_iterator
f17(const std::_Bit_const_iterator::const_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator-(a0, a1);
}

std::_Bit_const_iterator::const_iterator
f18(const std::_Bit_const_iterator::const_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator+(a0, a1);
}

using t6 = std::_Bit_iterator;

using t7 = std::_Bit_iterator::reference;

using t8 = std::_Bit_iterator::pointer;

using t9 = std::_Bit_iterator::iterator;

std::_Bit_iterator f19() { return std::_Bit_iterator(); }

std::_Bit_iterator f20(unsigned long *a0, unsigned int a1) {
  return std::_Bit_iterator(a0, a1);
}

std::_Bit_iterator::reference f21(const std::_Bit_iterator &a0) {
  return a0.operator*();
}

std::_Bit_iterator::iterator &f22(std::_Bit_iterator &a0) {
  return a0.operator++();
}

std::_Bit_iterator::iterator f23(std::_Bit_iterator &a0, int a1) {
  return a0.operator++(a1);
}

std::_Bit_iterator::iterator &f24(std::_Bit_iterator &a0) {
  return a0.operator--();
}

std::_Bit_iterator::iterator f25(std::_Bit_iterator &a0, int a1) {
  return a0.operator--(a1);
}

std::_Bit_iterator::iterator &
f26(std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator+=(a1);
}

std::_Bit_iterator::iterator &
f27(std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator-=(a1);
}

std::_Bit_iterator::reference
f28(const std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator[](a1);
}

std::_Bit_iterator f29(const std::_Bit_iterator &a0) {
  return std::_Bit_iterator(a0);
}

std::_Bit_iterator f30(std::_Bit_iterator &&a0) {
  return std::_Bit_iterator(std::move(a0));
}

std::_Bit_iterator &f31(std::_Bit_iterator &a0, const std::_Bit_iterator &a1) {
  return a0.operator=(a1);
}

std::_Bit_iterator &f32(std::_Bit_iterator &a0, std::_Bit_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::_Bit_iterator::iterator
f33(const std::_Bit_iterator::iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator-(a0, a1);
}

std::_Bit_iterator::iterator
f34(std::iterator<std::random_access_iterator_tag, bool>::difference_type a0,
    const std::_Bit_iterator::iterator &a1) {
  return operator+(a0, a1);
}

std::_Bit_iterator::iterator
f35(const std::_Bit_iterator::iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator+(a0, a1);
}

using t10 = std::_Bit_reference;

std::_Bit_reference f36(const std::_Bit_reference &a0) {
  return std::_Bit_reference(a0);
}

bool f37(const std::_Bit_reference &a0) { return static_cast<bool>(a0); }

std::_Bit_reference &f38(std::_Bit_reference &a0, bool a1) {
  return a0.operator=(a1);
}

const std::_Bit_reference &f39(const std::_Bit_reference &a0, bool a1) {
  return a0.operator=(a1);
}

std::_Bit_reference &f40(std::_Bit_reference &a0,
                         const std::_Bit_reference &a1) {
  return a0.operator=(a1);
}

bool f41(const std::_Bit_reference &a0, const std::_Bit_reference &a1) {
  return a0.operator==(a1);
}

bool f42(const std::_Bit_reference &a0, const std::_Bit_reference &a1) {
  return a0.operator<(a1);
}

void f43(std::_Bit_reference &a0) { return a0.flip(); }

void f44(bool &a0, std::_Bit_reference a1) { return swap(a0, std::move(a1)); }

void f45(std::_Bit_reference a0, bool &a1) { return swap(std::move(a0), a1); }

void f46(std::_Bit_reference a0, std::_Bit_reference a1) {
  return swap(std::move(a0), std::move(a1));
}
