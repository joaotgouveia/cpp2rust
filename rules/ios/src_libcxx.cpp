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

std::codecvt_base::result f7() { return std::codecvt_base::ok; }

std::codecvt_base::result f8() { return std::codecvt_base::partial; }

std::codecvt_base::result f9() { return std::codecvt_base::error; }

std::codecvt_base::result f10() { return std::codecvt_base::noconv; }

using t3 = std::ctype_base;

using t4 = std::ctype_base::mask;

std::ctype_base f11() { return std::ctype_base(); }

std::ctype_base f12(const std::ctype_base &a0) { return std::ctype_base(a0); }

std::ctype_base f13(std::ctype_base &&a0) {
  return std::ctype_base(std::move(a0));
}

std::ctype_base &f14(std::ctype_base &a0, const std::ctype_base &a1) {
  return a0.operator=(a1);
}

std::ctype_base &f15(std::ctype_base &a0, std::ctype_base &&a1) {
  return a0.operator=(std::move(a1));
}

std::ctype_base::mask f16() { return std::ctype_base::space; }

std::ctype_base::mask f17() { return std::ctype_base::print; }

std::ctype_base::mask f18() { return std::ctype_base::cntrl; }

std::ctype_base::mask f19() { return std::ctype_base::upper; }

std::ctype_base::mask f20() { return std::ctype_base::lower; }

std::ctype_base::mask f21() { return std::ctype_base::alpha; }

std::ctype_base::mask f22() { return std::ctype_base::digit; }

std::ctype_base::mask f23() { return std::ctype_base::punct; }

std::ctype_base::mask f24() { return std::ctype_base::xdigit; }

std::ctype_base::mask f25() { return std::ctype_base::blank; }

std::ctype_base::mask f26() { return std::ctype_base::alnum; }

std::ctype_base::mask f27() { return std::ctype_base::graph; }

std::ios_base &f28(std::ios_base &a0) { return std::dec(a0); }

std::ios_base &f29(std::ios_base &a0) { return std::defaultfloat(a0); }

std::ios_base &f30(std::ios_base &a0) { return std::fixed(a0); }

template <typename T1> bool f31(const std::locale &a0) {
  return std::has_facet<T1>(a0);
}

std::ios_base &f32(std::ios_base &a0) { return std::hex(a0); }

std::ios_base &f33(std::ios_base &a0) { return std::hexfloat(a0); }

std::ios_base &f34(std::ios_base &a0) { return std::internal(a0); }

using t5 = std::io_errc;

std::io_errc f35() { return std::io_errc::stream; }

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

std::ios_base::fmtflags f36(const std::ios_base &a0) { return a0.flags(); }

std::ios_base::fmtflags f37(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f38(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f39(std::ios_base &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f40(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f41(const std::ios_base &a0) { return a0.precision(); }

std::streamsize f42(std::ios_base &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f43(const std::ios_base &a0) { return a0.width(); }

std::streamsize f44(std::ios_base &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f45(std::ios_base &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

std::locale f46(const std::ios_base &a0) { return a0.getloc(); }

int f47() { return std::ios_base::xalloc(); }

long &f48(std::ios_base &a0, int a1) { return a0.iword(a1); }

void *&f49(std::ios_base &a0, int a1) { return a0.pword(a1); }

void f50(std::ios_base &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f51(bool a0) { return std::ios_base::sync_with_stdio(a0); }

std::ios_base::iostate f52(const std::ios_base &a0) { return a0.rdstate(); }

void f53(std::ios_base &a0, std::ios_base::iostate a1) { return a0.clear(a1); }

void f54(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f55(const std::ios_base &a0) { return a0.good(); }

bool f56(const std::ios_base &a0) { return a0.eof(); }

bool f57(const std::ios_base &a0) { return a0.fail(); }

bool f58(const std::ios_base &a0) { return a0.bad(); }

std::ios_base::iostate f59(const std::ios_base &a0) { return a0.exceptions(); }

void f60(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::ios_base::fmtflags f61() { return std::ios_base::boolalpha; }

std::ios_base::fmtflags f62() { return std::ios_base::dec; }

std::ios_base::fmtflags f63() { return std::ios_base::fixed; }

std::ios_base::fmtflags f64() { return std::ios_base::hex; }

std::ios_base::fmtflags f65() { return std::ios_base::internal; }

std::ios_base::fmtflags f66() { return std::ios_base::left; }

std::ios_base::fmtflags f67() { return std::ios_base::oct; }

std::ios_base::fmtflags f68() { return std::ios_base::right; }

std::ios_base::fmtflags f69() { return std::ios_base::scientific; }

std::ios_base::fmtflags f70() { return std::ios_base::showbase; }

std::ios_base::fmtflags f71() { return std::ios_base::showpoint; }

std::ios_base::fmtflags f72() { return std::ios_base::showpos; }

std::ios_base::fmtflags f73() { return std::ios_base::skipws; }

std::ios_base::fmtflags f74() { return std::ios_base::unitbuf; }

std::ios_base::fmtflags f75() { return std::ios_base::uppercase; }

std::ios_base::fmtflags f76() { return std::ios_base::adjustfield; }

std::ios_base::fmtflags f77() { return std::ios_base::basefield; }

std::ios_base::fmtflags f78() { return std::ios_base::floatfield; }

std::ios_base::iostate f79() { return std::ios_base::badbit; }

std::ios_base::iostate f80() { return std::ios_base::eofbit; }

std::ios_base::iostate f81() { return std::ios_base::failbit; }

std::ios_base::iostate f82() { return std::ios_base::goodbit; }

std::ios_base::openmode f83() { return std::ios_base::app; }

std::ios_base::openmode f84() { return std::ios_base::ate; }

std::ios_base::openmode f85() { return std::ios_base::binary; }

std::ios_base::openmode f86() { return std::ios_base::in; }

std::ios_base::openmode f87() { return std::ios_base::out; }

std::ios_base::openmode f88() { return std::ios_base::trunc; }

std::ios_base::failure f89(const std::string &a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

std::ios_base::failure f90(const char *a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

std::ios_base::failure f91(const std::ios_base::failure &a0) {
  return std::ios_base::failure(a0);
}

std::ios_base::failure &f92(std::ios_base::failure &a0,
                            const std::ios_base::failure &a1) {
  return a0.operator=(a1);
}

const std::error_code &f93(const std::ios_base::failure &a0) {
  return a0.code();
}

const char *f94(const std::ios_base::failure &a0) { return a0.what(); }

std::ios_base::Init f95() { return std::ios_base::Init(); }

std::ios_base::Init f96(const std::ios_base::Init &a0) {
  return std::ios_base::Init(a0);
}

std::ios_base::Init &f97(std::ios_base::Init &a0,
                         const std::ios_base::Init &a1) {
  return a0.operator=(a1);
}

std::ios_base::seekdir f98() { return std::ios_base::beg; }

std::ios_base::seekdir f99() { return std::ios_base::cur; }

std::ios_base::seekdir f100() { return std::ios_base::end; }

std::ios_base::event f101() { return std::ios_base::erase_event; }

std::ios_base::event f102() { return std::ios_base::imbue_event; }

std::ios_base::event f103() { return std::ios_base::copyfmt_event; }

const std::error_category &f104() { return std::iostream_category(); }

template <typename T1> bool f105(T1 a0, const std::locale &a1) {
  return std::isalnum<T1>(std::move(a0), a1);
}

template <typename T1> bool f106(T1 a0, const std::locale &a1) {
  return std::isalpha<T1>(std::move(a0), a1);
}

template <typename T1> bool f107(T1 a0, const std::locale &a1) {
  return std::isblank<T1>(std::move(a0), a1);
}

template <typename T1> bool f108(T1 a0, const std::locale &a1) {
  return std::iscntrl<T1>(std::move(a0), a1);
}

template <typename T1> bool f109(T1 a0, const std::locale &a1) {
  return std::isdigit<T1>(std::move(a0), a1);
}

template <typename T1> bool f110(T1 a0, const std::locale &a1) {
  return std::isgraph<T1>(std::move(a0), a1);
}

template <typename T1> bool f111(T1 a0, const std::locale &a1) {
  return std::islower<T1>(std::move(a0), a1);
}

template <typename T1> bool f112(T1 a0, const std::locale &a1) {
  return std::isprint<T1>(std::move(a0), a1);
}

template <typename T1> bool f113(T1 a0, const std::locale &a1) {
  return std::ispunct<T1>(std::move(a0), a1);
}

template <typename T1> bool f114(T1 a0, const std::locale &a1) {
  return std::isspace<T1>(std::move(a0), a1);
}

template <typename T1> bool f115(T1 a0, const std::locale &a1) {
  return std::isupper<T1>(std::move(a0), a1);
}

template <typename T1> bool f116(T1 a0, const std::locale &a1) {
  return std::isxdigit<T1>(std::move(a0), a1);
}

std::ios_base &f117(std::ios_base &a0) { return std::left(a0); }

using t16 = std::locale;

using t17 = std::locale::facet;

using t18 = std::locale::id;

using t19 = std::locale::category;

std::locale f118() { return std::locale(); }

std::locale f119(const std::locale &a0) { return std::locale(a0); }

std::locale f120(const char *a0) { return std::locale(a0); }

std::locale f121(const std::string &a0) { return std::locale(a0); }

std::locale f122(const std::locale &a0, const char *a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f123(const std::locale &a0, const std::string &a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

std::locale f124(const std::locale &a0, const std::locale &a1,
                 std::locale::category a2) {
  return std::locale(a0, a1, a2);
}

const std::locale &f125(std::locale &a0, const std::locale &a1) {
  return a0.operator=(a1);
}

std::string f126(const std::locale &a0) { return a0.name(); }

bool f127(const std::locale &a0, const std::locale &a1) {
  return a0.operator==(a1);
}

std::locale f128(const std::locale &a0) { return std::locale::global(a0); }

const std::locale &f129() { return std::locale::classic(); }

template <typename T1> std::locale f130(const std::locale &a0, T1 *a1) {
  return std::locale(a0, a1);
}

template <typename T1>
std::locale f131(const std::locale &a0, const std::locale &a1) {
  return a0.template combine<T1>(a1);
}

template <typename T1, typename T2, typename T3>
bool f132(const std::locale &a0, const std::basic_string<T1, T2, T3> &a1,
          const std::basic_string<T1, T2, T3> &a2) {
  return a0.template operator()<T1, T2, T3>(a1, a2);
}

std::locale::category f133() { return std::locale::none; }

std::locale::category f134() { return std::locale::collate; }

std::locale::category f135() { return std::locale::ctype; }

std::locale::category f136() { return std::locale::monetary; }

std::locale::category f137() { return std::locale::numeric; }

std::locale::category f138() { return std::locale::time; }

std::locale::category f139() { return std::locale::messages; }

std::locale::category f140() { return std::locale::all; }

long f141(const std::locale::facet &a0) { return a0.use_count(); }

std::locale::id f142() { return {}; }

std::error_code f143(std::io_errc a0) { return std::make_error_code(a0); }

std::error_condition f144(std::io_errc a0) {
  return std::make_error_condition(a0);
}

std::ios_base &f145(std::ios_base &a0) { return std::noboolalpha(a0); }

std::ios_base &f146(std::ios_base &a0) { return std::noshowbase(a0); }

std::ios_base &f147(std::ios_base &a0) { return std::noshowpoint(a0); }

std::ios_base &f148(std::ios_base &a0) { return std::noshowpos(a0); }

std::ios_base &f149(std::ios_base &a0) { return std::noskipws(a0); }

std::ios_base &f150(std::ios_base &a0) { return std::nounitbuf(a0); }

std::ios_base &f151(std::ios_base &a0) { return std::nouppercase(a0); }

std::ios_base &f152(std::ios_base &a0) { return std::oct(a0); }

std::ios_base &f153(std::ios_base &a0) { return std::right(a0); }

std::ios_base &f154(std::ios_base &a0) { return std::scientific(a0); }

std::ios_base &f155(std::ios_base &a0) { return std::showbase(a0); }

std::ios_base &f156(std::ios_base &a0) { return std::showpoint(a0); }

std::ios_base &f157(std::ios_base &a0) { return std::showpos(a0); }

std::ios_base &f158(std::ios_base &a0) { return std::skipws(a0); }

using t20 = std::streamoff;

using t21 = std::streamsize;

template <typename T1> T1 f159(T1 a0, const std::locale &a1) {
  return std::tolower<T1>(std::move(a0), a1);
}

template <typename T1> T1 f160(T1 a0, const std::locale &a1) {
  return std::toupper<T1>(std::move(a0), a1);
}

std::ios_base &f161(std::ios_base &a0) { return std::unitbuf(a0); }

std::ios_base &f162(std::ios_base &a0) { return std::uppercase(a0); }

template <typename T1> const T1 &f163(const std::locale &a0) {
  return std::use_facet<T1>(a0);
}

using t22 = std::wios;
