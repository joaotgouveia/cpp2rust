#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> std::atomic<T1 *> f1055() { return std::atomic<T1 *>(); }

template <typename T1> std::atomic<T1 *> f1056(T1 *a0) {
  return std::atomic<T1 *>(a0);
}

template <typename T1> std::atomic<T1> f1057() { return std::atomic<T1>(); }

template <typename T1> std::atomic<T1> f1058(T1 a0) {
  return std::atomic<T1>(std::move(a0));
}

template <typename T1>
bool f1059(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1060(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1061(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, std::move(a2),
                                                          a3, a4);
}

template <typename T1>
bool f1062(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, std::move(a2),
                                                          a3, a4);
}

template <typename T1>
bool f1063(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1064(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f1065(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, std::move(a2),
                                                        a3, a4);
}

template <typename T1>
bool f1066(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
           typename std::atomic<T1>::value_type a2, std::memory_order a3,
           std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, std::move(a2),
                                                        a3, a4);
}

template <typename T1>
T1 f1067(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, std::move(a1));
}

template <typename T1>
T1 f1068(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, std::move(a1));
}

template <typename T1>
T1 f1069(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
T1 f1070(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1071(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1072(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1073(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1074(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1075(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1076(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1077(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1078(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1079(volatile std::atomic<T1> *a0,
         typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1080(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, std::move(a1));
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1081(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f1082(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
void f1083(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, std::move(a1));
}

template <typename T1>
void f1084(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, std::move(a1));
}

template <typename T1>
void f1085(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, std::move(a1));
}

template <typename T1>
void f1086(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, std::move(a1));
}

template <typename T1>
void f1087(volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1>
void f1088(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
           std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, std::move(a1), a2);
}
