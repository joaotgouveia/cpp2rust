#include <flat_map>
#include <flat_set>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f3() {
  return std::flat_set<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f4(const std::flat_set<T1, T2, T3> &a0) {
  return std::flat_set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f5(std::flat_set<T1, T2, T3> &&a0) {
  return std::flat_set<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f6(const T2 &a0) {
  return std::flat_set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f7(T3 a0, const T2 &a1) {
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f8(std::sorted_unique_t a0, T3 a1, const T2 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f9(std::initializer_list<T1> a0, const T2 &a1) {
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> f10(std::sorted_unique_t a0,
                              std::initializer_list<T1> a1, const T2 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> &f11(std::flat_set<T1, T2, T3> &a0,
                               std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> &f12(std::flat_set<T1, T2, T3> &a0,
                               const std::flat_set<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::flat_set<T1, T2, T3> &f13(std::flat_set<T1, T2, T3> &a0,
                               std::flat_set<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f14(std::flat_set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f15(const std::flat_set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f16(std::flat_set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f17(const std::flat_set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::reverse_iterator
f18(std::flat_set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f19(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::reverse_iterator
f20(std::flat_set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f21(const std::flat_set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f22(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f23(const std::flat_set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f24(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_reverse_iterator
f25(const std::flat_set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3>
bool f26(const std::flat_set<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f27(const std::flat_set<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f28(const std::flat_set<T1, T2, T3> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator, bool>
f29(std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator, bool>
f30(std::flat_set<T1, T2, T3> &a0, T1 &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f31(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, const T1 &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f32(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T1 &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
void f33(std::flat_set<T1, T2, T3> &a0, std::initializer_list<T1> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3>
void f34(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1,
         std::initializer_list<T1> a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T3 f35(std::flat_set<T1, T2, T3> &&a0) {
  return std::move(a0).extract();
}

template <typename T1, typename T2, typename T3>
void f36(std::flat_set<T1, T2, T3> &a0, T3 &&a1) {
  return a0.replace(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f37(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::iterator a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type f38(std::flat_set<T1, T2, T3> &a0,
                                                  const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator
f39(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1,
    typename std::flat_set<T1, T2, T3>::const_iterator a2) {
  return a0.erase(a1, a2);
}

template <typename T1, typename T2, typename T3>
void f40(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
void f41(std::flat_set<T1, T2, T3> &a0) {
  return a0.clear();
}

template <typename T1, typename T2, typename T3>
T2 f42(const std::flat_set<T1, T2, T3> &a0) {
  return a0.key_comp();
}

template <typename T1, typename T2, typename T3>
T2 f43(const std::flat_set<T1, T2, T3> &a0) {
  return a0.value_comp();
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f44(std::flat_set<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f45(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::size_type
f46(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3>
bool f47(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f48(std::flat_set<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f49(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.lower_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::iterator f50(std::flat_set<T1, T2, T3> &a0,
                                                 const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
typename std::flat_set<T1, T2, T3>::const_iterator
f51(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.upper_bound(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f52(std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f53(const std::flat_set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f54(T4 a0, T4 a1, const T2 &a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f55(std::sorted_unique_t a0, T4 a1, T4 a2,
                              const T2 &a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f56(std::from_range_t a0, T4 &&a1) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f57(std::from_range_t a0, T4 &&a1, const T2 &a2) {
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f58(const T4 &a0)
  requires(std::uses_allocator<
              typename std::flat_set<T1, T2, T3>::container_type, T4>::value) &&
          (!(std::is_same<T4, std::flat_set<T1, T2, T3>>::value) &&
           !(std::is_same<
               T4, typename std::flat_set<T1, T2, T3>::key_compare>::value))
{
  return std::flat_set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f59(const T2 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f60(const T3 &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f61(const T3 &a0, const T2 &a1, const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f62(std::sorted_unique_t a0, const T3 &a1,
                              const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f63(std::sorted_unique_t a0, const T3 &a1,
                              const T2 &a2, const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f64(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f65(std::flat_set<T1, T2, T3> &&a0, const T4 &a1)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f66(T4 a0, T4 a1, const T5 &a2)
  requires((std::__has_input_iterator_category<T4>::value &&
            std::uses_allocator<
                typename std::flat_set<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f67(T4 a0, T4 a1, const T2 &a2, const T5 &a3)
  requires((std::__has_input_iterator_category<T4>::value &&
            std::uses_allocator<
                typename std::flat_set<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_set<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f68(std::sorted_unique_t a0, T4 a1, T4 a2,
                              const T5 &a3)
  requires((std::__has_input_iterator_category<T4>::value &&
            std::uses_allocator<
                typename std::flat_set<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f69(std::sorted_unique_t a0, T4 a1, T4 a2,
                              const T2 &a3, const T5 &a4)
  requires((std::__has_input_iterator_category<T4>::value &&
            std::uses_allocator<
                typename std::flat_set<T1, T2, T3>::container_type, T5>::value))
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f70(std::from_range_t a0, T4 &&a1, const T5 &a2)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T5>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::flat_set<T1, T2, T3> f71(std::from_range_t a0, T4 &&a1, const T2 &a2,
                              const T5 &a3)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T5>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f72(std::initializer_list<T1> a0, const T4 &a1)
  requires(std::uses_allocator<
              typename std::flat_set<T1, T2, T3>::container_type, T4>::value) &&
          (!(std::is_same<
              T4, typename std::flat_set<T1, T2, T3>::key_compare>::value))
{
  return std::flat_set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f73(std::initializer_list<T1> a0, const T2 &a1,
                              const T4 &a2)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f74(std::sorted_unique_t a0,
                              std::initializer_list<T1> a1, const T4 &a2)
  requires(std::uses_allocator<
              typename std::flat_set<T1, T2, T3>::container_type, T4>::value) &&
          (!(std::is_same<
              T4, typename std::flat_set<T1, T2, T3>::key_compare>::value))
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::flat_set<T1, T2, T3> f75(std::sorted_unique_t a0,
                              std::initializer_list<T1> a1, const T2 &a2,
                              const T4 &a3)
  requires(std::uses_allocator<
           typename std::flat_set<T1, T2, T3>::container_type, T4>::value)
{
  return std::flat_set<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator, bool>
f76(std::flat_set<T1, T2, T3> &a0, T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
typename std::flat_set<T1, T2, T3>::iterator
f77(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&...a2) {
  return a0.template emplace_hint<T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator, bool>
f78(std::flat_set<T1, T2, T3> &a0, T4 &&a1)
  requires((std::__is_transparent_v<T2> &&
            std::is_constructible_v<
                typename std::flat_set<T1, T2, T3>::value_type, T4>)) &&
          (!(std::is_same<T4, const typename std::flat_set<
                                  T1, T2, T3>::value_type &>::value) &&
           !(std::is_same<
               T4, typename std::flat_set<T1, T2, T3>::value_type>::value) &&
           !(std::is_same<T4, std::initializer_list<typename std::flat_set<
                                  T1, T2, T3>::value_type>>::value))
{
  return a0.template insert<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator
f79(std::flat_set<T1, T2, T3> &a0,
    typename std::flat_set<T1, T2, T3>::const_iterator a1, T4 &&a2)
  requires((std::__is_transparent_v<T2> &&
            std::is_constructible_v<
                typename std::flat_set<T1, T2, T3>::value_type, T4>)) &&
          (!(std::is_same<T4, const typename std::flat_set<
                                  T1, T2, T3>::value_type &>::value) &&
           !(std::is_same<
               T4, typename std::flat_set<T1, T2, T3>::value_type>::value))
{
  return a0.template insert<T4>(a1, std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f80(std::flat_set<T1, T2, T3> &a0, T4 a1, T4 a2)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f81(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1, T4 a2, T4 a3)
  requires(std::__has_input_iterator_category<T4>::value)
{
  return a0.template insert<T4>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f82(std::flat_set<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template insert_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f83(std::flat_set<T1, T2, T3> &a0, std::sorted_unique_t a1, T4 &&a2) {
  return a0.template insert_range<T4>(std::move(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type f84(std::flat_set<T1, T2, T3> &a0,
                                                  T4 &&a1)
  requires((std::__is_transparent_v<T2> &&
            !std::is_convertible_v<
                T4 &&, typename std::flat_set<T1, T2, T3>::iterator> &&
            !std::is_convertible_v<
                T4 &&, typename std::flat_set<T1, T2, T3>::const_iterator>)) &&
          (!(std::is_same<
               T4, typename std::flat_set<T1, T2, T3>::iterator>::value) &&
           !(std::is_same<T4, const typename std::flat_set<T1, T2, T3>::key_type
                                  &>::value))
{
  return a0.template erase<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f85(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f86(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::size_type
f87(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f88(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f89(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f90(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::iterator f91(std::flat_set<T1, T2, T3> &a0,
                                                 const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_set<T1, T2, T3>::const_iterator
f92(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::iterator,
          typename std::flat_set<T1, T2, T3>::iterator>
f93(std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<typename std::flat_set<T1, T2, T3>::const_iterator,
          typename std::flat_set<T1, T2, T3>::const_iterator>
f94(const std::flat_set<T1, T2, T3> &a0, const T4 &a1)
  requires(std::__is_transparent_v<T2>) &&
          (!(std::is_same<T4,
                          typename std::flat_set<T1, T2, T3>::key_type>::value))
{
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3>
void f95(std::flat_set<T1, T2, T3> &a0, std::flat_set<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f96(const std::flat_set<T1, T2, T3> &a0,
         const std::flat_set<T1, T2, T3> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f97(const std::flat_set<T1, T2, T3> &a0,
         const std::flat_set<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
using t15 = typename std::flat_set<T1, T2, T3>::iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t16 = typename std::flat_set<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::flat_set<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::flat_set<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t19 =
    typename std::flat_set<T1, T2, T3>::const_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t20 =
    typename std::flat_set<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t21 = typename std::flat_set<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t22 = typename std::flat_set<T1, T2, T3>::const_iterator::difference_type;
