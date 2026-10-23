#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f114(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f115(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f116(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f117(std::multimap<T1, T2, T3, T4> &a0,
          std::multimap<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f118(const T3 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f119(std::initializer_list<std::pair<const T1, T2>> a0,
                              const T3 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f120(T5 a0, T5 a1, const T3 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}
