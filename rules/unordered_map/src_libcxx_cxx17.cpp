#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, template <class, class> typename T3>
using t66 = std::__basic_node_handle<T1, T2, T3>;

template <typename T1, typename T2, template <class, class> typename T3>
using t67 = typename std::__basic_node_handle<T1, T2, T3>::allocator_type;

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3> f181() {
  return std::__basic_node_handle<T1, T2, T3>();
}

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3>
f182(std::__basic_node_handle<T1, T2, T3> &&a0) {
  return std::__basic_node_handle<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, template <class, class> typename T3>
std::__basic_node_handle<T1, T2, T3> &
f183(std::__basic_node_handle<T1, T2, T3> &a0,
     std::__basic_node_handle<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, template <class, class> typename T3>
T2 f184(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, template <class, class> typename T3>
bool f185(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, template <class, class> typename T3>
bool f186(const std::__basic_node_handle<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, template <class, class> typename T3>
void f187(std::__basic_node_handle<T1, T2, T3> &a0,
          std::__basic_node_handle<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, template <class, class> typename T3>
void f188(std::__basic_node_handle<T1, T2, T3> &a0,
          std::__basic_node_handle<T1, T2, T3> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2>
using t68 = std::__insert_return_type<T1, T2>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f189(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f190(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f191(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&...a2) {
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f192(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(std::move(a1), a2,
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f193(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&...a3) {
  return a0.template try_emplace<T6...>(std::move(a1), std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f194(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f195(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f196(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const T1 &a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(std::move(a1), a2,
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f197(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T1 &&a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::move(a2),
                                          std::forward<T6>(a3));
}
