#include <compare>
#include <stacktrace>
#include <utility>

template <typename T1> using t1 = std::basic_stacktrace<T1>;

template <typename T1>
using t2 = typename std::basic_stacktrace<T1>::value_type;

template <typename T1>
using t3 = typename std::basic_stacktrace<T1>::const_reference;

template <typename T1> using t4 = typename std::basic_stacktrace<T1>::reference;

template <typename T1>
using t5 = typename std::basic_stacktrace<T1>::const_iterator;

template <typename T1> using t6 = typename std::basic_stacktrace<T1>::iterator;

template <typename T1>
using t7 = typename std::basic_stacktrace<T1>::reverse_iterator;

template <typename T1>
using t8 = typename std::basic_stacktrace<T1>::const_reverse_iterator;

template <typename T1>
using t9 = typename std::basic_stacktrace<T1>::difference_type;

template <typename T1>
using t10 = typename std::basic_stacktrace<T1>::size_type;

template <typename T1>
using t11 = typename std::basic_stacktrace<T1>::allocator_type;

template <typename T1> std::basic_stacktrace<T1> f1(const T1 &a0) {
  return std::basic_stacktrace<T1>::current(a0);
}

template <typename T1>
std::basic_stacktrace<T1> f2(unsigned short a0, const T1 &a1) {
  return std::basic_stacktrace<T1>::current(a0, a1);
}

template <typename T1>
std::basic_stacktrace<T1> f3(unsigned short a0, unsigned short a1,
                             const T1 &a2) {
  return std::basic_stacktrace<T1>::current(a0, a1, a2);
}

template <typename T1> std::basic_stacktrace<T1> f4() {
  return std::basic_stacktrace<T1>();
}

template <typename T1> std::basic_stacktrace<T1> f5(const T1 &a0) {
  return std::basic_stacktrace<T1>(a0);
}

template <typename T1>
std::basic_stacktrace<T1> f6(const std::basic_stacktrace<T1> &a0) {
  return std::basic_stacktrace<T1>(a0);
}

template <typename T1>
std::basic_stacktrace<T1> f7(std::basic_stacktrace<T1> &&a0) {
  return std::basic_stacktrace<T1>(std::move(a0));
}

template <typename T1>
std::basic_stacktrace<T1> f8(const std::basic_stacktrace<T1> &a0,
                             const T1 &a1) {
  return std::basic_stacktrace<T1>(a0, a1);
}

template <typename T1>
std::basic_stacktrace<T1> f9(std::basic_stacktrace<T1> &&a0, const T1 &a1) {
  return std::basic_stacktrace<T1>(std::move(a0), a1);
}

template <typename T1>
std::basic_stacktrace<T1> &f10(std::basic_stacktrace<T1> &a0,
                               const std::basic_stacktrace<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::basic_stacktrace<T1> &f11(std::basic_stacktrace<T1> &a0,
                               std::basic_stacktrace<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f12(const std::basic_stacktrace<T1> &a0) {
  return a0.get_allocator();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_iterator
f13(const std::basic_stacktrace<T1> &a0) {
  return a0.begin();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_iterator
f14(const std::basic_stacktrace<T1> &a0) {
  return a0.end();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_reverse_iterator
f15(const std::basic_stacktrace<T1> &a0) {
  return a0.rbegin();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_reverse_iterator
f16(const std::basic_stacktrace<T1> &a0) {
  return a0.rend();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_iterator
f17(const std::basic_stacktrace<T1> &a0) {
  return a0.cbegin();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_iterator
f18(const std::basic_stacktrace<T1> &a0) {
  return a0.cend();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_reverse_iterator
f19(const std::basic_stacktrace<T1> &a0) {
  return a0.crbegin();
}

template <typename T1>
typename std::basic_stacktrace<T1>::const_reverse_iterator
f20(const std::basic_stacktrace<T1> &a0) {
  return a0.crend();
}

template <typename T1> bool f21(const std::basic_stacktrace<T1> &a0) {
  return a0.empty();
}

template <typename T1> unsigned short f22(const std::basic_stacktrace<T1> &a0) {
  return a0.size();
}

template <typename T1> unsigned short f23(const std::basic_stacktrace<T1> &a0) {
  return a0.max_size();
}

template <typename T1>
const std::stacktrace_entry &f24(const std::basic_stacktrace<T1> &a0,
                                 unsigned short a1) {
  return a0.operator[](a1);
}

template <typename T1>
const std::stacktrace_entry &f25(const std::basic_stacktrace<T1> &a0,
                                 unsigned short a1) {
  return a0.at(a1);
}

template <typename T1>
void f26(std::basic_stacktrace<T1> &a0, std::basic_stacktrace<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::strong_ordering f27(const std::basic_stacktrace<T1> &a0,
                         const std::basic_stacktrace<T2> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f28(const std::basic_stacktrace<T1> &a0,
         const std::basic_stacktrace<T2> &a1) {
  return operator==(a0, a1);
}
