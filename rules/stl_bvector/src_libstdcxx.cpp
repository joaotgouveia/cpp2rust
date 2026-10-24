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

std::_Bit_const_iterator::const_iterator f19() {
  return std::_Bit_const_iterator::const_iterator();
}

std::_Bit_const_iterator::const_iterator f20(unsigned long *a0,
                                             unsigned int a1) {
  return std::_Bit_const_iterator::const_iterator(a0, a1);
}

std::_Bit_const_iterator::const_iterator f21(const std::_Bit_iterator &a0) {
  return std::_Bit_const_iterator::const_iterator(a0);
}

bool f22(const std::_Bit_const_iterator::const_iterator &a0) {
  return a0.operator*();
}

std::_Bit_const_iterator::const_iterator &
f23(std::_Bit_const_iterator::const_iterator &a0) {
  return a0.operator++();
}

std::_Bit_const_iterator::const_iterator
f24(std::_Bit_const_iterator::const_iterator &a0, int a1) {
  return a0.operator++(a1);
}

std::_Bit_const_iterator::const_iterator &
f25(std::_Bit_const_iterator::const_iterator &a0) {
  return a0.operator--();
}

std::_Bit_const_iterator::const_iterator
f26(std::_Bit_const_iterator::const_iterator &a0, int a1) {
  return a0.operator--(a1);
}

std::_Bit_const_iterator::const_iterator &
f27(std::_Bit_const_iterator::const_iterator &a0, long a1) {
  return a0.operator+=(a1);
}

std::_Bit_const_iterator::const_iterator &
f28(std::_Bit_const_iterator::const_iterator &a0, long a1) {
  return a0.operator-=(a1);
}

bool f29(const std::_Bit_const_iterator::const_iterator &a0, long a1) {
  return a0.operator[](a1);
}

std::_Bit_const_iterator::const_iterator
f30(const std::_Bit_const_iterator::const_iterator &a0) {
  return std::_Bit_const_iterator::const_iterator(a0);
}

std::_Bit_const_iterator::const_iterator
f31(std::_Bit_const_iterator::const_iterator &&a0) {
  return std::_Bit_const_iterator::const_iterator(std::move(a0));
}

std::_Bit_const_iterator::const_iterator &
f32(std::_Bit_const_iterator::const_iterator &a0,
    const std::_Bit_const_iterator::const_iterator &a1) {
  return a0.operator=(a1);
}

std::_Bit_const_iterator::const_iterator &
f33(std::_Bit_const_iterator::const_iterator &a0,
    std::_Bit_const_iterator::const_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::_Bit_const_iterator::const_iterator
f34(long a0, const std::_Bit_const_iterator::const_iterator &a1) {
  return operator+(a0, a1);
}

std::_Bit_const_iterator::const_iterator
f35(const std::_Bit_const_iterator::const_iterator &a0, long a1) {
  return operator-(a0, a1);
}

std::_Bit_const_iterator::const_iterator
f36(const std::_Bit_const_iterator::const_iterator &a0, long a1) {
  return operator+(a0, a1);
}

using t6 = std::_Bit_iterator;

using t7 = std::_Bit_iterator::reference;

using t8 = std::_Bit_iterator::pointer;

using t9 = std::_Bit_iterator::iterator;

std::_Bit_iterator f37() { return std::_Bit_iterator(); }

std::_Bit_iterator f38(unsigned long *a0, unsigned int a1) {
  return std::_Bit_iterator(a0, a1);
}

std::_Bit_iterator::reference f39(const std::_Bit_iterator &a0) {
  return a0.operator*();
}

std::_Bit_iterator::iterator &f40(std::_Bit_iterator &a0) {
  return a0.operator++();
}

std::_Bit_iterator::iterator f41(std::_Bit_iterator &a0, int a1) {
  return a0.operator++(a1);
}

std::_Bit_iterator::iterator &f42(std::_Bit_iterator &a0) {
  return a0.operator--();
}

std::_Bit_iterator::iterator f43(std::_Bit_iterator &a0, int a1) {
  return a0.operator--(a1);
}

std::_Bit_iterator::iterator &
f44(std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator+=(a1);
}

std::_Bit_iterator::iterator &
f45(std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator-=(a1);
}

std::_Bit_iterator::reference
f46(const std::_Bit_iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return a0.operator[](a1);
}

std::_Bit_iterator f47(const std::_Bit_iterator &a0) {
  return std::_Bit_iterator(a0);
}

std::_Bit_iterator f48(std::_Bit_iterator &&a0) {
  return std::_Bit_iterator(std::move(a0));
}

std::_Bit_iterator &f49(std::_Bit_iterator &a0, const std::_Bit_iterator &a1) {
  return a0.operator=(a1);
}

std::_Bit_iterator &f50(std::_Bit_iterator &a0, std::_Bit_iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::_Bit_iterator::iterator
f51(const std::_Bit_iterator::iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator-(a0, a1);
}

std::_Bit_iterator::iterator
f52(std::iterator<std::random_access_iterator_tag, bool>::difference_type a0,
    const std::_Bit_iterator::iterator &a1) {
  return operator+(a0, a1);
}

std::_Bit_iterator::iterator
f53(const std::_Bit_iterator::iterator &a0,
    std::iterator<std::random_access_iterator_tag, bool>::difference_type a1) {
  return operator+(a0, a1);
}

std::_Bit_iterator::reference f54(const std::_Bit_iterator::reference &a0) {
  return std::_Bit_iterator::reference(a0);
}

bool f55(const std::_Bit_iterator::reference &a0) {
  return static_cast<bool>(a0);
}

std::_Bit_iterator::reference &f56(std::_Bit_iterator::reference &a0, bool a1) {
  return a0.operator=(a1);
}

std::_Bit_iterator::reference &f57(std::_Bit_iterator::reference &a0,
                                   const std::_Bit_iterator::reference &a1) {
  return a0.operator=(a1);
}

bool f58(const std::_Bit_iterator::reference &a0,
         const std::_Bit_iterator::reference &a1) {
  return a0.operator==(a1);
}

bool f59(const std::_Bit_iterator::reference &a0,
         const std::_Bit_iterator::reference &a1) {
  return a0.operator<(a1);
}

void f60(std::_Bit_iterator::reference &a0) { return a0.flip(); }

void f61(bool &a0, std::_Bit_iterator::reference a1) {
  return swap(a0, std::move(a1));
}

void f62(std::_Bit_iterator::reference a0, bool &a1) {
  return swap(std::move(a0), a1);
}

void f63(std::_Bit_iterator::reference a0, std::_Bit_iterator::reference a1) {
  return swap(std::move(a0), std::move(a1));
}

std::_Bit_iterator::iterator f64() { return std::_Bit_iterator::iterator(); }

std::_Bit_iterator::iterator f65(unsigned long *a0, unsigned int a1) {
  return std::_Bit_iterator::iterator(a0, a1);
}

std::_Bit_reference f66(const std::_Bit_iterator::iterator &a0) {
  return a0.operator*();
}

std::_Bit_iterator::iterator &f67(std::_Bit_iterator::iterator &a0) {
  return a0.operator++();
}

std::_Bit_iterator::iterator f68(std::_Bit_iterator::iterator &a0, int a1) {
  return a0.operator++(a1);
}

std::_Bit_iterator::iterator &f69(std::_Bit_iterator::iterator &a0) {
  return a0.operator--();
}

std::_Bit_iterator::iterator f70(std::_Bit_iterator::iterator &a0, int a1) {
  return a0.operator--(a1);
}

std::_Bit_iterator::iterator &f71(std::_Bit_iterator::iterator &a0, long a1) {
  return a0.operator+=(a1);
}

std::_Bit_iterator::iterator &f72(std::_Bit_iterator::iterator &a0, long a1) {
  return a0.operator-=(a1);
}

std::_Bit_reference f73(const std::_Bit_iterator::iterator &a0, long a1) {
  return a0.operator[](a1);
}

std::_Bit_iterator::iterator f74(const std::_Bit_iterator::iterator &a0) {
  return std::_Bit_iterator::iterator(a0);
}

std::_Bit_iterator::iterator f75(std::_Bit_iterator::iterator &&a0) {
  return std::_Bit_iterator::iterator(std::move(a0));
}

std::_Bit_iterator::iterator &f76(std::_Bit_iterator::iterator &a0,
                                  const std::_Bit_iterator::iterator &a1) {
  return a0.operator=(a1);
}

std::_Bit_iterator::iterator &f77(std::_Bit_iterator::iterator &a0,
                                  std::_Bit_iterator::iterator &&a1) {
  return a0.operator=(std::move(a1));
}

std::_Bit_iterator::iterator f78(const std::_Bit_iterator::iterator &a0,
                                 long a1) {
  return operator-(a0, a1);
}

std::_Bit_iterator::iterator f79(long a0,
                                 const std::_Bit_iterator::iterator &a1) {
  return operator+(a0, a1);
}

std::_Bit_iterator::iterator f80(const std::_Bit_iterator::iterator &a0,
                                 long a1) {
  return operator+(a0, a1);
}

using t10 = std::_Bit_reference;

std::_Bit_reference f81(const std::_Bit_reference &a0) {
  return std::_Bit_reference(a0);
}

bool f82(const std::_Bit_reference &a0) { return static_cast<bool>(a0); }

std::_Bit_reference &f83(std::_Bit_reference &a0, bool a1) {
  return a0.operator=(a1);
}

std::_Bit_reference &f84(std::_Bit_reference &a0,
                         const std::_Bit_reference &a1) {
  return a0.operator=(a1);
}

bool f85(const std::_Bit_reference &a0, const std::_Bit_reference &a1) {
  return a0.operator==(a1);
}

bool f86(const std::_Bit_reference &a0, const std::_Bit_reference &a1) {
  return a0.operator<(a1);
}

void f87(std::_Bit_reference &a0) { return a0.flip(); }

void f88(bool &a0, std::_Bit_reference a1) { return swap(a0, std::move(a1)); }

void f89(std::_Bit_reference a0, bool &a1) { return swap(std::move(a0), a1); }

void f90(std::_Bit_reference a0, std::_Bit_reference a1) {
  return swap(std::move(a0), std::move(a1));
}
