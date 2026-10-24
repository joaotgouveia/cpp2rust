#include <flat_map>
#include <flat_set>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f55(std::flat_multiset<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f56() {
  return std::flat_multiset<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f57(const std::flat_multiset<T1, T2, T3> &a0) {
  return std::flat_multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f58(std::flat_multiset<T1, T2, T3> &&a0) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f59(const T2 &a0) {
  return std::flat_multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f60(T3 a0, const T2 &a1) {
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f61(std::sorted_equivalent_t a0, T3 a1,
                                   const T2 &a2) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f62(std::initializer_list<T1> a0, const T2 &a1) {
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> f63(std::sorted_equivalent_t a0,
                                   std::initializer_list<T1> a1, const T2 &a2) {
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f64(std::flat_multiset<T1, T2, T3> &a0,
                                    std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f65(std::flat_multiset<T1, T2, T3> &a0,
                                    const std::flat_multiset<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_multiset<T1, T2, T3> &f66(std::flat_multiset<T1, T2, T3> &a0,
                                    std::flat_multiset<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f67(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f68(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::reverse_iterator
f69(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::reverse_iterator
f70(std::flat_multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f71(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f72(std::flat_multiset<T1, T2, T3> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f73(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::const_iterator a1, const T1 &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f74(std::flat_multiset<T1, T2, T3> &a0,
    typename std::flat_multiset<T1, T2, T3>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f75(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f76(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f77(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::size_type
f78(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f79(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f80(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f81(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::iterator
f82(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f83(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_multiset<T1, T2, T3>::iterator,
          typename std::flat_multiset<T1, T2, T3>::iterator>
f84(std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_multiset<T1, T2, T3>::const_iterator,
          typename std::flat_multiset<T1, T2, T3>::const_iterator>
f85(const std::flat_multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f86(T4 a0, T4 a1, const T2 &a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f87(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                   const T2 &a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f88(std::from_range_t a0, T4 &&a1)
  requires(std::_ContainerCompatibleRange<
           T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f89(std::from_range_t a0, T4 &&a1, const T2 &a2)
  requires(std::_ContainerCompatibleRange<
           T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                        a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f90(const T4 &a0)
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
std::flat_multiset<T1, T2, T3> f91(const T2 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f92(const T3 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f93(const T3 &a0, const T2 &a1, const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f94(std::sorted_equivalent_t a0, const T3 &a1,
                                   const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f95(std::sorted_equivalent_t a0, const T3 &a1,
                                   const T2 &a2, const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f96(const std::flat_multiset<T1, T2, T3> &a0,
                                   const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f97(std::flat_multiset<T1, T2, T3> &&a0,
                                   const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f98(T4 a0, T4 a1, const T5 &a2)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f99(T4 a0, T4 a1, const T2 &a2, const T5 &a3)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f100(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                    const T5 &a3)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f101(std::sorted_equivalent_t a0, T4 a1, T4 a2,
                                    const T2 &a3, const T5 &a4)
  requires(
      (std::__has_input_iterator_category<T4>::value &&
       std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f102(std::from_range_t a0, T4 &&a1, const T5 &a2)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T5>::value) &&
          (std::_ContainerCompatibleRange<
              T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                        a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_multiset<T1, T2, T3> f103(std::from_range_t a0, T4 &&a1, const T2 &a2,
                                    const T5 &a3)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T5>::value) &&
          (std::_ContainerCompatibleRange<
              T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2,
                                        a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f104(std::initializer_list<T1> a0, const T4 &a1)
  requires(std::uses_allocator<
              typename std::flat_multiset<T1, T2, T3>::container_type,
              T4>::value) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_compare>::value))
{
  return std::flat_multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f105(std::initializer_list<T1> a0, const T2 &a1,
                                    const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_multiset<T1, T2, T3> f106(std::sorted_equivalent_t a0,
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
std::flat_multiset<T1, T2, T3> f107(std::sorted_equivalent_t a0,
                                    std::initializer_list<T1> a1, const T2 &a2,
                                    const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_multiset<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_multiset<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f108(std::flat_multiset<T1, T2, T3> &a0, T4 &&...a1)
  requires(std::is_constructible_v<
           typename std::flat_multiset<T1, T2, T3>::value_type, T4...>)
{
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f109(std::flat_multiset<T1, T2, T3> &a0,
     typename std::flat_multiset<T1, T2, T3>::const_iterator a1, T4 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_multiset<T1, T2, T3>::value_type, T4...>)
{
  return a0.template emplace_hint<T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
void f110(std::flat_multiset<T1, T2, T3> &a0, T4 a1, T4 a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f111(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1,
          T4 a2, T4 a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f112(std::flat_multiset<T1, T2, T3> &a0, T4 &&a1)
  requires(std::_ContainerCompatibleRange<
           T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f113(std::flat_multiset<T1, T2, T3> &a0, std::sorted_equivalent_t a1,
          T4 &&a2)
  requires(std::_ContainerCompatibleRange<
           T4, typename std::flat_multiset<T1, T2, T3>::value_type>)
{
  return a0.template insert_range<T4>(std::move(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f114(std::flat_multiset<T1, T2, T3> &a0, T4 &&a1)
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
f115(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f116(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f117(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f118(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f119(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f120(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::iterator
f121(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::const_iterator
f122(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_multiset<T1, T2, T3>::iterator,
          typename std::flat_multiset<T1, T2, T3>::iterator>
f123(std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_multiset<T1, T2, T3>::const_iterator,
          typename std::flat_multiset<T1, T2, T3>::const_iterator>
f124(const std::flat_multiset<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<
              T4, typename std::flat_multiset<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f125(std::flat_multiset<T1, T2, T3> &a0,
          std::flat_multiset<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f126(const std::flat_multiset<T1, T2, T3> &a0,
          const std::flat_multiset<T1, T2, T3> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f127(const std::flat_multiset<T1, T2, T3> &a0,
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
