#include <initializer_list>
#include <map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f87(
    std::multimap<T1, T2, T3, T4> &a0,
    std::multimap<typename std::multimap<T1, T2, T3, T4>::key_type,
                  typename std::multimap<T1, T2, T3, T4>::mapped_type, T5,
                  typename std::multimap<T1, T2, T3, T4>::allocator_type> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f88(std::multimap<T1, T2, T3, T4> &a0,
         std::multimap<typename std::multimap<T1, T2, T3, T4>::key_type,
                       typename std::multimap<T1, T2, T3, T4>::mapped_type, T5,
                       typename std::multimap<T1, T2, T3, T4>::allocator_type>
             &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f89(std::multimap<T1, T2, T3, T4> &a0,
         std::map<typename std::multimap<T1, T2, T3, T4>::key_type,
                  typename std::multimap<T1, T2, T3, T4>::mapped_type, T5,
                  typename std::multimap<T1, T2, T3, T4>::allocator_type> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f90(
    std::multimap<T1, T2, T3, T4> &a0,
    std::map<typename std::multimap<T1, T2, T3, T4>::key_type,
             typename std::multimap<T1, T2, T3, T4>::mapped_type, T5,
             typename std::multimap<T1, T2, T3, T4>::allocator_type> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}
