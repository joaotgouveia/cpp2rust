#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1>
void f1165(const volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f1166(volatile std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1167(volatile std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 *f1168(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f1169(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f1170(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f1171(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f1172(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 *f1173(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.fetch_xor(a1, a2);
}

template <typename T1>
T1 *f1174(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 *f1175(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator&=(a1);
}

template <typename T1>
T1 *f1176(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 *f1177(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator|=(a1);
}

template <typename T1>
T1 *f1178(volatile std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
T1 *f1179(std::atomic<T1 *> &a0, T1 *a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.operator^=(a1);
}

template <typename T1>
bool f1180(const volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f1181(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f1182(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
void f1183(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f1184(const volatile std::atomic<T1 *> &a0, std::memory_order a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 *f1185(const std::atomic<T1 *> &a0, std::memory_order a1)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 *f1186(const volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f1187(const std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return static_cast<T1 *>(a0);
}

template <typename T1>
T1 *f1188(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f1189(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f1190(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3, std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f1191(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f1192(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3, std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f1193(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f1194(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f1195(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f1196(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f1197(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
void f1198(const volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f1199(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.wait(a1, a2);
}

template <typename T1>
void f1200(volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1201(std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1202(volatile std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f1203(std::atomic<T1 *> &a0)
  requires(std::is_integral<T1 *>::value &&
           !std::is_same<T1 *, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
  requires(std::is_floating_point_v<T1>)
using t78 = std::atomic<T1>;

template <typename T1> using t79 = typename std::atomic<T1>::value_type;

template <typename T1> using t80 = typename std::atomic<T1>::difference_type;

template <typename T1>
std::atomic<T1> f1204()
  requires(std::is_floating_point_v<T1>)
{
  return {};
}

template <typename T1>
std::atomic<T1> f1205(T1 a0)
  requires(std::is_floating_point_v<T1>)
{
  return {std::move(a0)};
}

template <typename T1>
T1 f1206(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 f1207(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 f1208(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f1209(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f1210(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f1211(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f1212(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f1213(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f1214(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::atomic<T1>::__base::is_always_lock_free) &&
          (std::is_floating_point_v<T1>)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f1215(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
bool f1216(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f1217(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.is_lock_free();
}

template <typename T1>
void f1218(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f1219(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f1220(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1221(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1222(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1223(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1224(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f1225(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f1226(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1227(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1228(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1229(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1230(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1231(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1232(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f1233(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f1234(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1235(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1236(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f1237(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_one();
}

template <typename T1>
void f1238(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_all();
}

template <typename T1>
void f1239(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>)
{
  return a0.notify_all();
}

template <typename T1>
T1 f1240(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f1241(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f1242(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f1243(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f1244(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f1245(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f1246(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f1247(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f1248(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f1249(std::atomic<T1> &a0, int a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f1250(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f1251(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f1252(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f1253(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f1254(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f1255(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f1256(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f1257(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f1258(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
T1 f1259(std::atomic<T1> &a0, T1 a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
bool f1260(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f1261(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f1262(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f1263(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f1264(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1265(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1266(const volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1267(const std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1268(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f1269(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f1270(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1271(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1272(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1273(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1274(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1275(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1276(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f1277(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f1278(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1279(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1280(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1281(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1282(volatile std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f1283(std::atomic<T1> &a0)
  requires(std::is_floating_point_v<T1>) &&
          (std::is_integral<T1>::value &&
           !std::is_same<T1, bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f1284(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f1285(volatile std::atomic<T1> &a0) {
  return a0.notify_one();
}

template <typename T1> void f1286(volatile std::atomic<T1> &a0) {
  return a0.notify_all();
}

template <typename T1>
T1 f1287(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f1288(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_add(std::move(a1), a2);
}

template <typename T1>
T1 f1289(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f1290(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_sub(std::move(a1), a2);
}

template <typename T1>
T1 f1291(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f1292(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_and(std::move(a1), a2);
}

template <typename T1>
T1 f1293(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f1294(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_or(std::move(a1), a2);
}

template <typename T1>
T1 f1295(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f1296(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.fetch_xor(std::move(a1), a2);
}

template <typename T1>
T1 f1297(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f1298(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++(a1);
}

template <typename T1>
T1 f1299(volatile std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f1300(std::atomic<T1> &a0, int a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--(a1);
}

template <typename T1>
T1 f1301(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f1302(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator++();
}

template <typename T1>
T1 f1303(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f1304(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator--();
}

template <typename T1>
T1 f1305(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f1306(std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator+=(std::move(a1));
}

template <typename T1>
T1 f1307(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f1308(std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator-=(std::move(a1));
}

template <typename T1>
T1 f1309(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f1310(std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator&=(std::move(a1));
}

template <typename T1>
T1 f1311(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f1312(std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator|=(std::move(a1));
}

template <typename T1>
T1 f1313(volatile std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
T1 f1314(std::atomic<T1> &a0, T1 a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.operator^=(std::move(a1));
}

template <typename T1>
bool f1315(const volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
bool f1316(const std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.is_lock_free();
}

template <typename T1>
void f1317(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f1318(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.store(std::move(a1), a2);
}

template <typename T1>
T1 f1319(const volatile std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1320(const std::atomic<T1> &a0, std::memory_order a1)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.load(a1);
}

template <typename T1>
T1 f1321(const volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1322(const std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f1323(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f1324(std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f1325(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1326(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1327(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1328(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f1329(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1330(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f1331(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f1332(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
void f1333(const volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1334(const std::atomic<T1> &a0, T1 a1, std::memory_order a2)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.wait(std::move(a1), a2);
}

template <typename T1>
void f1335(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1336(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_one();
}

template <typename T1>
void f1337(volatile std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1>
void f1338(std::atomic<T1> &a0)
  requires(std::is_integral<
               typename std::__check_atomic_mandates<T1>::type>::value &&
           !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                         bool>::value == true)
{
  return a0.notify_all();
}

template <typename T1> void f1339(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f1340(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f1341(const volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, std::move(a1));
}

template <typename T1>
void f1342(const volatile std::atomic<T1> *a0,
           typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, std::move(a1), a2);
}
