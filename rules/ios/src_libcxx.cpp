#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

std::ios_base &f1(std::ios_base &a0) { return std::boolalpha(a0); }

using t1 = std::codecvt_base;

using t2 = std::codecvt_base::result;

std::codecvt_base f2() { return std::codecvt_base(); }

std::codecvt_base f3(const std::codecvt_base &a0) {
  return std::codecvt_base(a0);
}

std::codecvt_base f4(std::codecvt_base &&a0) {
  return std::codecvt_base(std::move(a0));
}

std::codecvt_base &f5(std::codecvt_base &a0, const std::codecvt_base &a1) {
  return a0.operator=(a1);
}

std::codecvt_base &f6(std::codecvt_base &a0, std::codecvt_base &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::ctype_base;

using t4 = std::ctype_base::mask;

std::ctype_base f7() { return std::ctype_base(); }

std::ctype_base f8(const std::ctype_base &a0) { return std::ctype_base(a0); }

std::ctype_base f9(std::ctype_base &&a0) {
  return std::ctype_base(std::move(a0));
}

std::ctype_base &f10(std::ctype_base &a0, const std::ctype_base &a1) {
  return a0.operator=(a1);
}

std::ctype_base &f11(std::ctype_base &a0, std::ctype_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::ctype_base::mask f12() { return std::ctype_base::space; }

std::ctype_base::mask f13() { return std::ctype_base::print; }

std::ctype_base::mask f14() { return std::ctype_base::cntrl; }

std::ctype_base::mask f15() { return std::ctype_base::upper; }

std::ctype_base::mask f16() { return std::ctype_base::lower; }

std::ctype_base::mask f17() { return std::ctype_base::alpha; }

std::ctype_base::mask f18() { return std::ctype_base::digit; }

std::ctype_base::mask f19() { return std::ctype_base::punct; }

std::ctype_base::mask f20() { return std::ctype_base::xdigit; }

std::ctype_base::mask f21() { return std::ctype_base::blank; }

std::ctype_base::mask f22() { return std::ctype_base::alnum; }

std::ctype_base::mask f23() { return std::ctype_base::graph; }

std::ios_base &f24(std::ios_base &a0) { return std::dec(a0); }

std::ios_base &f25(std::ios_base &a0) { return std::defaultfloat(a0); }

std::ios_base &f26(std::ios_base &a0) { return std::fixed(a0); }

template <typename T1> bool f27(const std::locale &a0) {
  return std::has_facet<T1>(a0);
}

std::ios_base &f28(std::ios_base &a0) { return std::hex(a0); }

std::ios_base &f29(std::ios_base &a0) { return std::hexfloat(a0); }

std::ios_base &f30(std::ios_base &a0) { return std::internal(a0); }

using t5 = std::io_errc;

std::io_errc f31() { return std::io_errc::stream; }

using t6 = std::ios;

using t7 = std::ios_base;

using t8 = std::ios_base::failure;

using t9 = std::ios_base::fmtflags;

using t10 = std::ios_base::iostate;

using t11 = std::ios_base::openmode;

using t12 = std::ios_base::seekdir;

using t13 = std::ios_base::Init;

using t14 = std::ios_base::event;

using t15 = std::ios_base::event_callback;

std::ios_base::fmtflags f32(const std::ios_base &a0) { return a0.flags(); }

std::ios_base::fmtflags f33(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f34(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f35(std::ios_base &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f36(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f37(const std::ios_base &a0) { return a0.precision(); }

std::streamsize f38(std::ios_base &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f39(const std::ios_base &a0) { return a0.width(); }

std::streamsize f40(std::ios_base &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f41(std::ios_base &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

std::locale f42(const std::ios_base &a0) { return a0.getloc(); }

int f43() { return std::ios_base::xalloc(); }

long &f44(std::ios_base &a0, int a1) { return a0.iword(a1); }

void *&f45(std::ios_base &a0, int a1) { return a0.pword(a1); }

void f46(std::ios_base &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f47(bool a0) { return std::ios_base::sync_with_stdio(a0); }

std::ios_base::iostate f48(const std::ios_base &a0) { return a0.rdstate(); }

void f49(std::ios_base &a0, std::ios_base::iostate a1) { return a0.clear(a1); }

void f50(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f51(const std::ios_base &a0) { return a0.good(); }

bool f52(const std::ios_base &a0) { return a0.eof(); }

bool f53(const std::ios_base &a0) { return a0.fail(); }

bool f54(const std::ios_base &a0) { return a0.bad(); }

std::ios_base::iostate f55(const std::ios_base &a0) { return a0.exceptions(); }

void f56(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::ios_base::fmtflags f57() { return std::ios_base::boolalpha; }

std::ios_base::fmtflags f58() { return std::ios_base::dec; }

std::ios_base::fmtflags f59() { return std::ios_base::fixed; }

std::ios_base::fmtflags f60() { return std::ios_base::hex; }

std::ios_base::fmtflags f61() { return std::ios_base::internal; }

std::ios_base::fmtflags f62() { return std::ios_base::left; }

std::ios_base::fmtflags f63() { return std::ios_base::oct; }

std::ios_base::fmtflags f64() { return std::ios_base::right; }

std::ios_base::fmtflags f65() { return std::ios_base::scientific; }

std::ios_base::fmtflags f66() { return std::ios_base::showbase; }

std::ios_base::fmtflags f67() { return std::ios_base::showpoint; }

std::ios_base::fmtflags f68() { return std::ios_base::showpos; }

std::ios_base::fmtflags f69() { return std::ios_base::skipws; }

std::ios_base::fmtflags f70() { return std::ios_base::unitbuf; }

std::ios_base::fmtflags f71() { return std::ios_base::uppercase; }

std::ios_base::fmtflags f72() { return std::ios_base::adjustfield; }

std::ios_base::fmtflags f73() { return std::ios_base::basefield; }

std::ios_base::fmtflags f74() { return std::ios_base::floatfield; }

std::ios_base::iostate f75() { return std::ios_base::badbit; }

std::ios_base::iostate f76() { return std::ios_base::eofbit; }

std::ios_base::iostate f77() { return std::ios_base::failbit; }

std::ios_base::iostate f78() { return std::ios_base::goodbit; }

std::ios_base::openmode f79() { return std::ios_base::app; }

std::ios_base::openmode f80() { return std::ios_base::ate; }

std::ios_base::openmode f81() { return std::ios_base::binary; }

std::ios_base::openmode f82() { return std::ios_base::in; }

std::ios_base::openmode f83() { return std::ios_base::out; }

std::ios_base::openmode f84() { return std::ios_base::trunc; }

std::ios_base::failure f85(const std::string &a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

std::ios_base::failure f86(const char *a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

std::ios_base::failure f87(const std::ios_base::failure &a0) {
  return std::ios_base::failure(a0);
}

std::ios_base::failure &f88(std::ios_base::failure &a0,
                            const std::ios_base::failure &a1) {
  return a0.operator=(a1);
}

const std::error_code &f89(const std::ios_base::failure &a0) {
  return a0.code();
}

const char *f90(const std::ios_base::failure &a0) { return a0.what(); }

std::ios_base::Init f91() { return std::ios_base::Init(); }

std::ios_base::Init f92(const std::ios_base::Init &a0) {
  return std::ios_base::Init(a0);
}

std::ios_base::Init &f93(std::ios_base::Init &a0,
                         const std::ios_base::Init &a1) {
  return a0.operator=(a1);
}

const std::error_category &f94() { return std::iostream_category(); }

template <typename T1> bool f95(T1 a0, const std::locale &a1) {
  return std::isalnum<T1>(a0, a1);
}

template <typename T1> bool f96(T1 a0, const std::locale &a1) {
  return std::isalpha<T1>(a0, a1);
}

template <typename T1> bool f97(T1 a0, const std::locale &a1) {
  return std::isblank<T1>(a0, a1);
}

template <typename T1> bool f98(T1 a0, const std::locale &a1) {
  return std::iscntrl<T1>(a0, a1);
}

template <typename T1> bool f99(T1 a0, const std::locale &a1) {
  return std::isdigit<T1>(a0, a1);
}

template <typename T1> bool f100(T1 a0, const std::locale &a1) {
  return std::isgraph<T1>(a0, a1);
}

template <typename T1> bool f101(T1 a0, const std::locale &a1) {
  return std::islower<T1>(a0, a1);
}

template <typename T1> bool f102(T1 a0, const std::locale &a1) {
  return std::isprint<T1>(a0, a1);
}

template <typename T1> bool f103(T1 a0, const std::locale &a1) {
  return std::ispunct<T1>(a0, a1);
}

template <typename T1> bool f104(T1 a0, const std::locale &a1) {
  return std::isspace<T1>(a0, a1);
}

template <typename T1> bool f105(T1 a0, const std::locale &a1) {
  return std::isupper<T1>(a0, a1);
}

template <typename T1> bool f106(T1 a0, const std::locale &a1) {
  return std::isxdigit<T1>(a0, a1);
}

std::ios_base &f107(std::ios_base &a0) { return std::left(a0); }

using t16 = std::locale;

using t17 = std::locale::facet;

using t18 = std::locale::id;

using t19 = std::locale::category;

std::locale f108() { return std::locale(); }

std::locale f109(const std::locale &a0) { return std::locale(a0); }

std::locale f110(const char *a0) { return std::locale(a0); }

std::locale f111(const std::string &a0) { return std::locale(a0); }

std::locale f112(const std::locale &a0, const char *a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f113(const std::locale &a0, const std::string &a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f114(const std::locale &a0, const std::locale &a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

const std::locale &f115(std::locale &a0, const std::locale &a1) {
  return a0.operator=(a1);
}

std::string f116(const std::locale &a0) { return a0.name(); }

bool f117(const std::locale &a0, const std::locale &a1) {
  return a0.operator==(a1);
}

std::locale f118(const std::locale &a0) { return std::locale::global(a0); }

const std::locale &f119() { return std::locale::classic(); }

template <typename T1> std::locale f120(const std::locale &a0, T1 *a1) {
  return std::locale(a0, a1);
}

template <typename T1>
std::locale f121(const std::locale &a0, const std::locale &a1) {
  return a0.template combine<T1>(a1);
}

template <typename T1, typename T2, typename T3>
bool f122(const std::locale &a0, const std::basic_string<T1, T2, T3> &a1,
          const std::basic_string<T1, T2, T3> &a2) {
  return a0.template operator()<T1, T2, T3>(a1, a2);
}

std::locale::category f123() { return std::locale::none; }

std::locale::category f124() { return std::locale::collate; }

std::locale::category f125() { return std::locale::ctype; }

std::locale::category f126() { return std::locale::monetary; }

std::locale::category f127() { return std::locale::numeric; }

std::locale::category f128() { return std::locale::time; }

std::locale::category f129() { return std::locale::messages; }

std::locale::category f130() { return std::locale::all; }

long f131(const std::locale::facet &a0) { return a0.use_count(); }

std::locale::id f132() { return {}; }

std::error_code f133(std::io_errc a0) { return std::make_error_code(a0); }

std::error_condition f134(std::io_errc a0) {
  return std::make_error_condition(a0);
}

std::ios_base &f135(std::ios_base &a0) { return std::noboolalpha(a0); }

std::ios_base &f136(std::ios_base &a0) { return std::noshowbase(a0); }

std::ios_base &f137(std::ios_base &a0) { return std::noshowpoint(a0); }

std::ios_base &f138(std::ios_base &a0) { return std::noshowpos(a0); }

std::ios_base &f139(std::ios_base &a0) { return std::noskipws(a0); }

std::ios_base &f140(std::ios_base &a0) { return std::nounitbuf(a0); }

std::ios_base &f141(std::ios_base &a0) { return std::nouppercase(a0); }

std::ios_base &f142(std::ios_base &a0) { return std::oct(a0); }

std::ios_base &f143(std::ios_base &a0) { return std::right(a0); }

std::ios_base &f144(std::ios_base &a0) { return std::scientific(a0); }

std::ios_base &f145(std::ios_base &a0) { return std::showbase(a0); }

std::ios_base &f146(std::ios_base &a0) { return std::showpoint(a0); }

std::ios_base &f147(std::ios_base &a0) { return std::showpos(a0); }

std::ios_base &f148(std::ios_base &a0) { return std::skipws(a0); }

using t20 = std::streamoff;

using t21 = std::streamsize;

template <typename T1> T1 f149(T1 a0, const std::locale &a1) {
  return std::tolower<T1>(a0, a1);
}

template <typename T1> T1 f150(T1 a0, const std::locale &a1) {
  return std::toupper<T1>(a0, a1);
}

std::ios_base &f151(std::ios_base &a0) { return std::unitbuf(a0); }

std::ios_base &f152(std::ios_base &a0) { return std::uppercase(a0); }

template <typename T1> const T1 &f153(const std::locale &a0) {
  return std::use_facet<T1>(a0);
}

using t22 = std::wios;
