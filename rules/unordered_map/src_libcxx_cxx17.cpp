#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, template <class, class> typename T3>
using t54 = std::__basic_node_handle<T1, T2, T3>;

template <typename T1, typename T2, template <class, class> typename T3>
using t55 = typename std::__basic_node_handle<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3> f167() {
  return std::__basic_node_handle<T1, T2, T3>();
}

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3>
f168(std::__basic_node_handle<T1, T2, T3> &&a0) {
  return std::__basic_node_handle<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3> &
f169(std::__basic_node_handle<T1, T2, T3> &a0,
     std::__basic_node_handle<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, template <class, class> typename T3>
T2 f170(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, template <class, class> typename T3>
bool f171(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, template <class, class> typename T3>
bool f172(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, template <class, class> typename T3>
void f173(std::__basic_node_handle<T1, T2, T3> &a0,
          std::__basic_node_handle<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, template <class, class> typename T3>
void f174(std::__basic_node_handle<T1, T2, T3> &a0,
          std::__basic_node_handle<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2>
using t56 = std::__insert_return_type<T1, T2>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f175(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f176(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f177(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f178(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(std::move(a1), a2,
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f179(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(std::move(a1), std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f180(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f181(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f182(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(std::move(a1), a2,
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f183(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::move(a2),
                                          std::forward<T6>(a3));
}
