#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f248(const typename std::basic_string<T1, T2, T3>::allocator_type &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f249(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f250(std::basic_string<T1, T2, T3> &&a0,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f251(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f252(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f253(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f254(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
void f255(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::size_type a1,
          typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f256(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f257(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f258(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f259(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f260(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f261(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f262(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f263(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f264(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f265(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f266(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f267(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
void f268(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f269(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f270(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f271(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f272(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f273(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f274(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f275(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f276(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f277(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f278(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     const typename std::basic_string<T1, T2, T3>::value_type *a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f279(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     const typename std::basic_string<T1, T2, T3>::value_type *a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f280(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::value_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f281(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f282(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::value_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f283(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f284(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f285(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f286(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::value_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f287(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f288(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f289(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::value_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f290(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f291(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type *
f292(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type *
f293(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f294(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f295(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f296(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f297(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f298(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f299(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f300(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f301(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f302(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f303(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f304(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f305(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f306(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f307(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f308(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f309(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f310(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f311(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f312(const std::basic_string<T1, T2, T3> &a0,
         const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f313(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f314(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const typename std::basic_string<T1, T2, T3>::value_type *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3>
f315(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3>
f316(const T4 &a0,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3>
f317(T4 a0, T4 a1,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f318(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f319(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f320(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f321(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f322(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f323(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f324(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f325(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f326(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f327(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f328(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f329(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f330(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template rfind<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f331(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f332(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f333(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f334(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
int f335(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
int f336(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
int f337(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
using t37 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t38 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t39 =
    typename std::basic_string<T1, T2,
                               T3>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t40 =
    typename std::basic_string<T1, T2, T3>::const_reverse_iterator::value_type;
