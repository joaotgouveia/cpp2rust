#include "../common/init.h"
#include "../common/traits.h"
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <utility>

template <typename N,
          typename =
              decltype(std::declval<N &>().key(), std::declval<N &>().mapped(),
                       std::declval<N &>().get_allocator(), void())>
using t1 = N;

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
using t2 = N;

template <typename N, typename = decltype(std::declval<N &>().position,
                                          std::declval<N &>().inserted,
                                          std::declval<N &>().node, void())>
using t3 = N;

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
N f1(N &&a0) {
  return N(std::move(a0));
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
N &f2(N &a0, N &&a1) {
  return a0 = std::move(a1);
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
typename N::allocator_type f3(const N &a0) {
  return a0.get_allocator();
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
bool f4(const N &a0) {
  return static_cast<bool>(a0);
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
bool f5(const N &a0) {
  return a0.empty();
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
void f6(N &a0, N &a1) {
  return a0.swap(a1);
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
void f7(N &a0, N &a1) {
  return swap(a0, a1);
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
typename N::key_type &f8(const N &a0) {
  return a0.key();
}

template <typename N,
          typename = decltype(std::declval<N &>().key(),
                              std::declval<N &>().mapped(),
                              std::declval<N &>().get_allocator(), void())>
typename N::mapped_type &f9(const N &a0) {
  return a0.mapped();
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
N f10(N &&a0) {
  return N(std::move(a0));
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
N &f11(N &a0, N &&a1) {
  return a0 = std::move(a1);
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
typename N::allocator_type f12(const N &a0) {
  return a0.get_allocator();
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
bool f13(const N &a0) {
  return static_cast<bool>(a0);
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
bool f14(const N &a0) {
  return a0.empty();
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
void f15(N &a0, N &a1) {
  return a0.swap(a1);
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
void f16(N &a0, N &a1) {
  return swap(a0, a1);
}

template <typename N, typename = decltype(std::declval<N &>().value(),
                                          std::declval<N &>().get_allocator(),
                                          std::declval<N &>().empty(), void())>
typename N::value_type &f17(const N &a0) {
  return a0.value();
}

template <typename N, typename = decltype(std::declval<N &>().position,
                                          std::declval<N &>().inserted,
                                          std::declval<N &>().node, void())>
auto f18(N &a0) -> decltype((a0.inserted)) {
  return a0.inserted;
}

template <typename N, typename = decltype(std::declval<N &>().position,
                                          std::declval<N &>().inserted,
                                          std::declval<N &>().node, void())>
auto f19(N &a0) -> decltype((a0.position)) {
  return a0.position;
}

template <typename N, typename = decltype(std::declval<N &>().position,
                                          std::declval<N &>().inserted,
                                          std::declval<N &>().node, void())>
auto f20(N &a0) -> decltype((a0.node)) {
  return a0.node;
}
