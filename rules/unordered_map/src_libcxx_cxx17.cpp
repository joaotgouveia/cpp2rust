#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2>
using t54 = std::__insert_return_type<T1, T2>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f167(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f168(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f169(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f170(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(a1, a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f171(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(a1, std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f172(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f173(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f174(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(a1, a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f175(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(a1, std::move(a2),
                                          std::forward<T6>(a3));
}
