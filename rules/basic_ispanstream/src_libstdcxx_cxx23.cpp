#include <ios>
#include <iosfwd>
#include <span>
#include <spanstream>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_ispanstream<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_ispanstream<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::basic_ispanstream<T1, T2>::int_type;

template <typename T1, typename T2>
using t4 = typename std::basic_ispanstream<T1, T2>::pos_type;

template <typename T1, typename T2>
using t5 = typename std::basic_ispanstream<T1, T2>::off_type;

template <typename T1, typename T2>
using t6 = typename std::basic_ispanstream<T1, T2>::traits_type;

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2>
f1(std::span<T1> a0, typename std::basic_ispanstream<T1, T2>::openmode a1) {
  return std::basic_ispanstream<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> f2(std::basic_ispanstream<T1, T2> &&a0) {
  return std::basic_ispanstream<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f3(std::basic_ispanstream<T1, T2> &a0,
                                   std::basic_ispanstream<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f4(std::basic_ispanstream<T1, T2> &a0,
        std::basic_ispanstream<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::basic_spanbuf<T1, T2> *f5(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::span<const T1> f6(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.span();
}

template <typename T1, typename T2>
void f7(std::basic_ispanstream<T1, T2> &a0, std::span<T1> a1) {
  return a0.span(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ispanstream<T1, T2> f8(T3 &&a0)
  requires(std::ranges::borrowed_range<T3> &&
           (!std::convertible_to<T3, std::span<T1>>) &&
           std::convertible_to<T3, std::span<const T1>>) &&
          (!(std::is_same<T3, const std::basic_ispanstream<T1, T2> &>::value) &&
           !(std::is_same<T3, std::basic_ispanstream<T1, T2>>::value))
{
  return std::basic_ispanstream<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3>
void f9(std::basic_ispanstream<T1, T2> &a0, T3 &&a1)
  requires(std::ranges::borrowed_range<T3> &&
           (!std::convertible_to<T3, std::span<T1>>) &&
           std::convertible_to<T3, std::span<const T1>>) &&
          (!(std::is_same<T3, std::span<T1>>::value))
{
  return a0.template span<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &
f10(std::basic_ispanstream<T1, T2> &a0,
    std::basic_ispanstream<T1, T2> &(*a1)(std::basic_ispanstream<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &
f11(std::basic_ispanstream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f12(std::basic_ispanstream<T1, T2> &a0,
                                    std::ios_base &(*a1)(std::ios_base &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f13(std::basic_ispanstream<T1, T2> &a0,
                                    bool &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f14(std::basic_ispanstream<T1, T2> &a0,
                                    short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f15(std::basic_ispanstream<T1, T2> &a0,
                                    unsigned short &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f16(std::basic_ispanstream<T1, T2> &a0,
                                    int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f17(std::basic_ispanstream<T1, T2> &a0,
                                    unsigned int &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f18(std::basic_ispanstream<T1, T2> &a0,
                                    long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f19(std::basic_ispanstream<T1, T2> &a0,
                                    unsigned long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f20(std::basic_ispanstream<T1, T2> &a0,
                                    long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f21(std::basic_ispanstream<T1, T2> &a0,
                                    unsigned long long &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f22(std::basic_ispanstream<T1, T2> &a0,
                                    float &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f23(std::basic_ispanstream<T1, T2> &a0,
                                    double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f24(std::basic_ispanstream<T1, T2> &a0,
                                    long double &a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f25(std::basic_ispanstream<T1, T2> &a0,
                                    void *&a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f26(std::basic_ispanstream<T1, T2> &a0,
                                    std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::streamsize f27(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.gcount();
}

template <typename T1, typename T2>
typename std::basic_ispanstream<T1, T2>::int_type
f28(std::basic_ispanstream<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f29(std::basic_ispanstream<T1, T2> &a0,
                                    T1 &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f30(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                                    std::streamsize a2, T1 a3) {
  return a0.get(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f31(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                                    std::streamsize a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f32(std::basic_ispanstream<T1, T2> &a0,
                                    std::basic_streambuf<T1, T2> &a1, T1 a2) {
  return a0.get(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f33(std::basic_ispanstream<T1, T2> &a0,
                                    std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f34(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                                    std::streamsize a2, T1 a3) {
  return a0.getline(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f35(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                                    std::streamsize a2) {
  return a0.getline(a1, a2);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &
f36(std::basic_ispanstream<T1, T2> &a0, std::streamsize a1,
    typename std::basic_ispanstream<T1, T2>::int_type a2) {
  return a0.ignore(a1, std::move(a2));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f37(std::basic_ispanstream<T1, T2> &a0,
                                    std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f38(std::basic_ispanstream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
typename std::basic_ispanstream<T1, T2>::int_type
f39(std::basic_ispanstream<T1, T2> &a0) {
  return a0.peek();
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f40(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                                    std::streamsize a2) {
  return a0.read(a1, a2);
}

template <typename T1, typename T2>
std::streamsize f41(std::basic_ispanstream<T1, T2> &a0, T1 *a1,
                    std::streamsize a2) {
  return a0.readsome(a1, a2);
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f42(std::basic_ispanstream<T1, T2> &a0, T1 a1) {
  return a0.putback(std::move(a1));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &f43(std::basic_ispanstream<T1, T2> &a0) {
  return a0.unget();
}

template <typename T1, typename T2>
int f44(std::basic_ispanstream<T1, T2> &a0) {
  return a0.sync();
}

template <typename T1, typename T2>
typename std::basic_ispanstream<T1, T2>::pos_type
f45(std::basic_ispanstream<T1, T2> &a0) {
  return a0.tellg();
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &
f46(std::basic_ispanstream<T1, T2> &a0,
    typename std::basic_ispanstream<T1, T2>::pos_type a1) {
  return a0.seekg(std::move(a1));
}

template <typename T1, typename T2>
std::basic_ispanstream<T1, T2> &
f47(std::basic_ispanstream<T1, T2> &a0,
    typename std::basic_ispanstream<T1, T2>::off_type a1,
    typename std::basic_ispanstream<T1, T2>::seekdir a2) {
  return a0.seekg(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
bool f48(const std::basic_ispanstream<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f49(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
typename std::basic_ispanstream<T1, T2>::iostate
f50(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.rdstate();
}

template <typename T1, typename T2>
void f51(std::basic_ispanstream<T1, T2> &a0,
         typename std::basic_ispanstream<T1, T2>::iostate a1) {
  return a0.clear(std::move(a1));
}

template <typename T1, typename T2>
void f52(std::basic_ispanstream<T1, T2> &a0,
         typename std::basic_ispanstream<T1, T2>::iostate a1) {
  return a0.setstate(std::move(a1));
}

template <typename T1, typename T2>
bool f53(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.good();
}

template <typename T1, typename T2>
bool f54(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.eof();
}

template <typename T1, typename T2>
bool f55(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.fail();
}

template <typename T1, typename T2>
bool f56(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.bad();
}

template <typename T1, typename T2>
typename std::basic_ispanstream<T1, T2>::iostate
f57(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.exceptions();
}

template <typename T1, typename T2>
void f58(std::basic_ispanstream<T1, T2> &a0,
         typename std::basic_ispanstream<T1, T2>::iostate a1) {
  return a0.exceptions(std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f59(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f60(std::basic_ispanstream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
T1 f61(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.fill();
}

template <typename T1, typename T2>
T1 f62(std::basic_ispanstream<T1, T2> &a0, T1 a1) {
  return a0.fill(std::move(a1));
}

template <typename T1, typename T2>
std::locale f63(std::basic_ispanstream<T1, T2> &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

template <typename T1, typename T2>
char f64(const std::basic_ispanstream<T1, T2> &a0, T1 a1, char a2) {
  return a0.narrow(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 f65(const std::basic_ispanstream<T1, T2> &a0, char a1) {
  return a0.widen(a1);
}

template <typename T1, typename T2>
void f66(std::basic_ispanstream<T1, T2> &a0,
         void (*a1)(typename std::basic_ispanstream<T1, T2>::event,
                    std::basic_ispanstream<T1, T2> &, int),
         int a2) {
  return a0.register_callback(a1, a2);
}

template <typename T1, typename T2>
std::ios_base::fmtflags f67(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.flags();
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f68(std::basic_ispanstream<T1, T2> &a0,
    typename std::basic_ispanstream<T1, T2>::fmtflags a1) {
  return a0.flags(std::move(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f69(std::basic_ispanstream<T1, T2> &a0,
    typename std::basic_ispanstream<T1, T2>::fmtflags a1) {
  return a0.setf(std::move(a1));
}

template <typename T1, typename T2>
std::ios_base::fmtflags
f70(std::basic_ispanstream<T1, T2> &a0,
    typename std::basic_ispanstream<T1, T2>::fmtflags a1,
    typename std::basic_ispanstream<T1, T2>::fmtflags a2) {
  return a0.setf(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f71(std::basic_ispanstream<T1, T2> &a0,
         typename std::basic_ispanstream<T1, T2>::fmtflags a1) {
  return a0.unsetf(std::move(a1));
}

template <typename T1, typename T2>
std::streamsize f72(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.precision();
}

template <typename T1, typename T2>
std::streamsize f73(std::basic_ispanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.precision(a1);
}

template <typename T1, typename T2>
std::streamsize f74(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.width();
}

template <typename T1, typename T2>
std::streamsize f75(std::basic_ispanstream<T1, T2> &a0, std::streamsize a1) {
  return a0.width(a1);
}

template <typename T1, typename T2> bool f76(bool a0) {
  return std::basic_ispanstream<T1, T2>::sync_with_stdio(a0);
}

template <typename T1, typename T2>
std::locale f77(const std::basic_ispanstream<T1, T2> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2> int f78() {
  return std::basic_ispanstream<T1, T2>::xalloc();
}

template <typename T1, typename T2>
long &f79(std::basic_ispanstream<T1, T2> &a0, int a1) {
  return a0.iword(a1);
}

template <typename T1, typename T2>
void *&f80(std::basic_ispanstream<T1, T2> &a0, int a1) {
  return a0.pword(a1);
}
