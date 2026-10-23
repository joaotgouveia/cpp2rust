#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f242(const typename std::basic_string<T1, T2, T3>::allocator_type &a0) {
  return std::basic_string<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f243(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f244(std::basic_string<T1, T2, T3> &&a0,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a1) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f245(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f246(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f247(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f248(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
void f249(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::size_type a1,
          typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f250(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f251(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f252(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f253(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f254(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f255(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f256(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f257(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f258(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f259(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.append(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f260(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.append(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f261(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.append(a1);
}

template <typename T1, typename T2, typename T3>
void f262(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.push_back(a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f263(std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f264(const std::basic_string<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::value_type &
f265(std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type &
f266(const std::basic_string<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f267(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f268(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f269(std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.assign(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f270(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f271(std::basic_string<T1, T2, T3> &a0,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a1) {
  return a0.assign(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f272(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     const typename std::basic_string<T1, T2, T3>::value_type *a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f273(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     const typename std::basic_string<T1, T2, T3>::value_type *a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f274(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::value_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f275(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::value_type a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f276(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::value_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::iterator
f277(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f278(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f279(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f280(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::value_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f281(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f282(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f283(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::value_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> &
f284(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2,
     std::initializer_list<typename std::basic_string<T1, T2, T3>::value_type>
         a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f285(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type *
f286(const std::basic_string<T1, T2, T3> &a0) {
  return a0.c_str();
}

template <typename T1, typename T2, typename T3>
const typename std::basic_string<T1, T2, T3>::value_type *
f287(const std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f288(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f289(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f290(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f291(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f292(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f293(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f294(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f295(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f296(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f297(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f298(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f299(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f300(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f301(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f302(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f303(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f304(const std::basic_string<T1, T2, T3> &a0,
     const typename std::basic_string<T1, T2, T3>::value_type *a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::size_type
f305(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::value_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2, typename T3>
int f306(const std::basic_string<T1, T2, T3> &a0,
         const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2, typename T3>
int f307(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const typename std::basic_string<T1, T2, T3>::value_type *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
int f308(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2,
         const typename std::basic_string<T1, T2, T3>::value_type *a3,
         typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
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
