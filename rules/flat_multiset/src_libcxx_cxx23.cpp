#include <flat_map>
#include <flat_set>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f1(std::flat_multiset<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f2() {
  return std::flat_multiset<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f3(const std::flat_multiset<T1, T2, T3> &a0) {
  return std::flat_multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f4(std::flat_multiset<T1, T2, T3> &&a0) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f5(const T2 &a0) {
  return std::flat_multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f6(T3 a0, const T2 &a1) {
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f7(std::sorted_equivalent_t a0, T3 a1,
                                  const T2 &a2) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f8(std::initializer_list<T1> a0, const T2 &a1) {
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f9(std::sorted_equivalent_t a0,
                                  std::initializer_list<T1> a1, const T2 &a2) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f10(std::flat_multiset<T1, T2, T3> &a0,
                                    std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f11(std::flat_multiset<T1, T2, T3> &a0,
                                    const std::flat_multiset<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f12(std::flat_multiset<T1, T2, T3> &a0,
                                    std::flat_multiset<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f13(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f14(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f15(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f16(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::reverse_iterator
f17(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f18(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::reverse_iterator
f19(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f20(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f21(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f22(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f23(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_reverse_iterator
f24(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f25(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f26(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f27(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f28(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f29(std::flat_multiset<T1, T2, T3> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f30(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1, const T1 &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f31(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f32(std::flat_multiset<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
void f33(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1,
         std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T3 f34(std::flat_multiset<T1, T2, T3> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3>
void f35(std::flat_multiset<T1, T2, T3> &a0, T3 &&a1) {
  return a0.replace(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f36(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f37(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f38(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f39(std::flat_multiset<T1, T2, T3> &a0,
         std::flat_multiset<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
void f40(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
T2 f41(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f42(const std::flat_multiset<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f43(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f44(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f45(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f46(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f47(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f48(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f49(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f50(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_multiset<T1, T2, T3>::iterator,
          typename std::flat_multiset<T1, T2, T3>::iterator>
f51(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_multiset<T1, T2, T3>::const_iterator,
          typename std::flat_multiset<T1, T2, T3>::const_iterator>
f52(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f53(T4 a0, T4 a1, const T2 &a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f54(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                   const T2 &a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f55(std::from_range_t a0, T4 &&a1) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f56(std::from_range_t a0, T4 &&a1,
                                   const T2 &a2) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                        a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f57(const T4 &a0)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T4>::value) &&
          (!(std::is_same<T4, std::flat_multiset<T1, T2, T3>>::value) &&
           !(std::is_same<T4, typename std::flat_multiset<
                                  T1, T2, T3>::key_compare>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f58(const T2 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f59(const T3 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f60(const T3 &a0, const T2 &a1, const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f61(std::sorted_equivalent_t a0, const T3 &a1,
                                   const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f62(std::sorted_equivalent_t a0, const T3 &a1,
                                   const T2 &a2, const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f63(const std::flat_multiset<T1, T2, T3> &a0,
                                   const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f64(std::flat_multiset<T1, T2, T3> &&a0,
                                   const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f65(T4 a0, T4 a1, const T5 &a2)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f66(T4 a0, T4 a1, const T2 &a2, const T5 &a3)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f67(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                   const T5 &a3)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f68(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                   const T2 &a3, const T5 &a4)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f69(std::from_range_t a0, T4 &&a1, const T5 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                        a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f70(std::from_range_t a0, T4 &&a1, const T2 &a2,
                                   const T5 &a3)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2,
                                        a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f71(std::initializer_list<T1> a0, const T4 &a1)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T4>::value) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_compare>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f72(std::initializer_list<T1> a0, const T2 &a1,
                                   const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f73(std::sorted_equivalent_t a0,
                                   std::initializer_list<T1> a1, const T4 &a2)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T4>::value) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_compare>::value))
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f74(std::sorted_equivalent_t a0,
                                   std::initializer_list<T1> a1, const T2 &a2,
                                   const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f75(std::flat_multiset<T1, T2, T3> &a0, T4 &&...a1)
  requires(std::is_constructible_v<
           typename std::flat_multiset<T1, T2, T3>::value_type, T4...>)
{
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f76(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1, T4 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_multiset<T1, T2, T3>::value_type, T4...>)
{
  return a0.template emplace_hint<T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f77(std::flat_multiset<T1, T2, T3> &a0, T4 a1, T4 a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f78(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1, T4 a2,
         T4 a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f79(std::flat_multiset<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f80(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1,
         T4 &&a2) {
  return a0.template insert_range<T4>(std::move(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f81(std::flat_multiset<T1, T2, T3> &a0, T4 &&a1)
  requires((std::__is_transparent_v<T2> &&
            !std::is_convertible_v<
                T4 &&, typename std::flat_multiset<T1, T2, T3>::iterator> &&
            !std::is_convertible_v<T4 &&, typename std::flat_multiset<
                                              T1, T2, T3>::const_iterator>)) &&
          (!(std::is_same<
               T4, typename std::flat_multiset<T1, T2, T3>::iterator>::value) &&
           !(std::is_same<T4, const typename std::flat_multiset<
                                  T1, T2, T3>::key_type &>::value))
{
  return a0.template erase<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f82(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f83(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f84(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f85(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f86(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f87(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f88(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f89(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_multiset<T1, T2, T3>::iterator,
          typename std::flat_multiset<T1, T2, T3>::iterator>
f90(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_multiset<T1, T2, T3>::const_iterator,
          typename std::flat_multiset<T1, T2, T3>::const_iterator>
f91(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f92(std::flat_multiset<T1, T2, T3> &a0,
         std::flat_multiset<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f93(const std::flat_multiset<T1, T2, T3> &a0,
         const std::flat_multiset<T1, T2, T3> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f94(const std::flat_multiset<T1, T2, T3> &a0,
         const std::flat_multiset<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
using t15 = typename std::flat_multiset<T1, T2, T3>::iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t16 =
    typename std::flat_multiset<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::flat_multiset<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::flat_multiset<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t19 =
    typename std::flat_multiset<T1, T2, T3>::const_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t20 =
    typename std::flat_multiset<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t21 = typename std::flat_multiset<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t22 =
    typename std::flat_multiset<T1, T2, T3>::const_iterator::difference_type;
