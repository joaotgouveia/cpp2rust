#include <ios>
#include <iosfwd>
#include <utility>

std::ios_base &f1(std::ios_base &a0) { return std::boolalpha(a0); }

std::ios_base &f2(std::ios_base &a0) { return std::dec(a0); }

std::ios_base &f3(std::ios_base &a0) { return std::defaultfloat(a0); }

std::ios_base &f4(std::ios_base &a0) { return std::fixed(a0); }

std::ios_base &f5(std::ios_base &a0) { return std::hex(a0); }

std::ios_base &f6(std::ios_base &a0) { return std::hexfloat(a0); }

std::ios_base &f7(std::ios_base &a0) { return std::internal(a0); }

using t1 = std::io_errc;

std::io_errc f8() { return std::io_errc::stream; }

using t2 = std::ios_base;

using t3 = std::ios_base::failure;

using t4 = std::ios_base::fmtflags;

using t5 = std::ios_base::iostate;

using t6 = std::ios_base::openmode;

using t7 = std::ios_base::seekdir;

using t8 = std::ios_base::event;

using t9 = std::ios_base::event_callback;

using t10 = std::ios_base::Init;

void f9(std::ios_base &a0,
        void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

std::ios_base::fmtflags f10(const std::ios_base &a0) { return a0.flags(); }

std::ios_base::fmtflags f11(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f12(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f13(std::ios_base &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f14(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f15(const std::ios_base &a0) { return a0.precision(); }

std::streamsize f16(std::ios_base &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f17(const std::ios_base &a0) { return a0.width(); }

std::streamsize f18(std::ios_base &a0, std::streamsize a1) {
  return a0.width(a1);
}

bool f19(bool a0) { return std::ios_base::sync_with_stdio(a0); }

std::locale f20(std::ios_base &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

std::locale f21(const std::ios_base &a0) { return a0.getloc(); }

int f22() { return std::ios_base::xalloc(); }

long &f23(std::ios_base &a0, int a1) { return a0.iword(a1); }

void *&f24(std::ios_base &a0, int a1) { return a0.pword(a1); }

std::ios_base::fmtflags f25() { return std::ios_base::boolalpha; }

std::ios_base::fmtflags f26() { return std::ios_base::dec; }

std::ios_base::fmtflags f27() { return std::ios_base::fixed; }

std::ios_base::fmtflags f28() { return std::ios_base::hex; }

std::ios_base::fmtflags f29() { return std::ios_base::internal; }

std::ios_base::fmtflags f30() { return std::ios_base::left; }

std::ios_base::fmtflags f31() { return std::ios_base::oct; }

std::ios_base::fmtflags f32() { return std::ios_base::right; }

std::ios_base::fmtflags f33() { return std::ios_base::scientific; }

std::ios_base::fmtflags f34() { return std::ios_base::showbase; }

std::ios_base::fmtflags f35() { return std::ios_base::showpoint; }

std::ios_base::fmtflags f36() { return std::ios_base::showpos; }

std::ios_base::fmtflags f37() { return std::ios_base::skipws; }

std::ios_base::fmtflags f38() { return std::ios_base::unitbuf; }

std::ios_base::fmtflags f39() { return std::ios_base::uppercase; }

std::ios_base::fmtflags f40() { return std::ios_base::adjustfield; }

std::ios_base::fmtflags f41() { return std::ios_base::basefield; }

std::ios_base::fmtflags f42() { return std::ios_base::floatfield; }

std::ios_base::iostate f43() { return std::ios_base::badbit; }

std::ios_base::iostate f44() { return std::ios_base::eofbit; }

std::ios_base::iostate f45() { return std::ios_base::failbit; }

std::ios_base::iostate f46() { return std::ios_base::goodbit; }

std::ios_base::openmode f47() { return std::ios_base::app; }

std::ios_base::openmode f48() { return std::ios_base::ate; }

std::ios_base::openmode f49() { return std::ios_base::binary; }

std::ios_base::openmode f50() { return std::ios_base::in; }

std::ios_base::openmode f51() { return std::ios_base::out; }

std::ios_base::openmode f52() { return std::ios_base::trunc; }

std::ios_base::openmode f53() { return std::ios_base::noreplace; }

std::ios_base::seekdir f54() { return std::ios_base::beg; }

std::ios_base::seekdir f55() { return std::ios_base::cur; }

std::ios_base::seekdir f56() { return std::ios_base::end; }

std::ios_base::failure f57(const std::string &a0) {
  return std::ios_base::failure(a0);
}

std::ios_base::failure f58(const std::string &a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

std::ios_base::failure f59(const char *a0, const std::error_code &a1) {
  return std::ios_base::failure(a0, a1);
}

const char *f60(const std::ios_base::failure &a0) { return a0.what(); }

std::ios_base::failure f61(const std::ios_base::failure &a0) {
  return std::ios_base::failure(a0);
}

std::ios_base::failure &f62(std::ios_base::failure &a0,
                            const std::ios_base::failure &a1) {
  return a0.operator=(a1);
}

const std::error_code &f63(const std::ios_base::failure &a0) {
  return a0.code();
}

std::ios_base::Init f64() { return std::ios_base::Init(); }

std::ios_base::Init f65(const std::ios_base::Init &a0) {
  return std::ios_base::Init(a0);
}

std::ios_base::Init &f66(std::ios_base::Init &a0,
                         const std::ios_base::Init &a1) {
  return a0.operator=(a1);
}

std::ios_base::event f67() { return std::ios_base::erase_event; }

std::ios_base::event f68() { return std::ios_base::imbue_event; }

std::ios_base::event f69() { return std::ios_base::copyfmt_event; }

const std::error_category &f70() { return std::iostream_category(); }

std::ios_base &f71(std::ios_base &a0) { return std::left(a0); }

std::error_code f72(std::io_errc a0) { return std::make_error_code(a0); }

std::error_condition f73(std::io_errc a0) {
  return std::make_error_condition(a0);
}

std::ios_base &f74(std::ios_base &a0) { return std::noboolalpha(a0); }

std::ios_base &f75(std::ios_base &a0) { return std::noshowbase(a0); }

std::ios_base &f76(std::ios_base &a0) { return std::noshowpoint(a0); }

std::ios_base &f77(std::ios_base &a0) { return std::noshowpos(a0); }

std::ios_base &f78(std::ios_base &a0) { return std::noskipws(a0); }

std::ios_base &f79(std::ios_base &a0) { return std::nounitbuf(a0); }

std::ios_base &f80(std::ios_base &a0) { return std::nouppercase(a0); }

std::ios_base &f81(std::ios_base &a0) { return std::oct(a0); }

std::ios_base::fmtflags f82(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator&(a0, a1);
}

std::ios_base::iostate f83(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator&(a0, a1);
}

std::ios_base::openmode f84(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator&(a0, a1);
}

const std::ios_base::fmtflags &f85(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::iostate &f86(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::openmode &f87(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator&=(a0, a1);
}

std::ios_base::fmtflags f88(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator^(a0, a1);
}

std::ios_base::iostate f89(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator^(a0, a1);
}

std::ios_base::openmode f90(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator^(a0, a1);
}

const std::ios_base::fmtflags &f91(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::iostate &f92(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::openmode &f93(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator^=(a0, a1);
}

std::ios_base::fmtflags f94(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator|(a0, a1);
}

std::ios_base::iostate f95(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator|(a0, a1);
}

std::ios_base::openmode f96(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator|(a0, a1);
}

const std::ios_base::fmtflags &f97(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::iostate &f98(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::openmode &f99(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator|=(a0, a1);
}

std::ios_base::fmtflags f100(std::ios_base::fmtflags a0) {
  return std::operator~(a0);
}

std::ios_base::iostate f101(std::ios_base::iostate a0) {
  return std::operator~(a0);
}

std::ios_base::openmode f102(std::ios_base::openmode a0) {
  return std::operator~(a0);
}

std::ios_base &f103(std::ios_base &a0) { return std::right(a0); }

std::ios_base &f104(std::ios_base &a0) { return std::scientific(a0); }

std::ios_base &f105(std::ios_base &a0) { return std::showbase(a0); }

std::ios_base &f106(std::ios_base &a0) { return std::showpoint(a0); }

std::ios_base &f107(std::ios_base &a0) { return std::showpos(a0); }

std::ios_base &f108(std::ios_base &a0) { return std::skipws(a0); }

std::ios_base &f109(std::ios_base &a0) { return std::unitbuf(a0); }

std::ios_base &f110(std::ios_base &a0) { return std::uppercase(a0); }
