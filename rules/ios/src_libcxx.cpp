#include <ios>
#include <iosfwd>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::ios_base &f1(std::ios_base &a0) { return std::boolalpha(a0); }

std::ios_base &f2(std::ios_base &a0) { return std::dec(a0); }

std::ios_base &f3(std::ios_base &a0) { return std::defaultfloat(a0); }

std::ios_base &f4(std::ios_base &a0) { return std::fixed(a0); }

std::ios_base &f5(std::ios_base &a0) { return std::hex(a0); }

std::ios_base &f6(std::ios_base &a0) { return std::hexfloat(a0); }

std::ios_base &f7(std::ios_base &a0) { return std::internal(a0); }

using t1 = std::io_errc;

std::io_errc f8() { return std::io_errc::stream; }

using t2 = std::ios;

using t3 = std::ios_base;

using t4 = std::ios_base::failure;

using t5 = std::ios_base::fmtflags;

using t6 = std::ios_base::iostate;

using t7 = std::ios_base::openmode;

using t8 = std::ios_base::seekdir;

using t9 = std::ios_base::Init;

using t10 = std::ios_base::event;

using t11 = std::ios_base::event_callback;

std::ios_base::fmtflags f9(const std::ios_base &a0) { return a0.flags(); }

std::ios_base::fmtflags f10(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.flags(a1);
}

std::ios_base::fmtflags f11(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.setf(a1);
}

std::ios_base::fmtflags f12(std::ios_base &a0, std::ios_base::fmtflags a1,
                            std::ios_base::fmtflags a2) {
  return a0.setf(a1, a2);
}

void f13(std::ios_base &a0, std::ios_base::fmtflags a1) {
  return a0.unsetf(a1);
}

std::streamsize f14(const std::ios_base &a0) { return a0.precision(); }

std::streamsize f15(std::ios_base &a0, std::streamsize a1) {
  return a0.precision(a1);
}

std::streamsize f16(const std::ios_base &a0) { return a0.width(); }

std::streamsize f17(std::ios_base &a0, std::streamsize a1) {
  return a0.width(a1);
}

std::locale f18(std::ios_base &a0, const std::locale &a1) {
  return a0.imbue(a1);
}

std::locale f19(const std::ios_base &a0) { return a0.getloc(); }

int f20() { return std::ios_base::xalloc(); }

long &f21(std::ios_base &a0, int a1) { return a0.iword(a1); }

void *&f22(std::ios_base &a0, int a1) { return a0.pword(a1); }

void f23(std::ios_base &a0,
         void (*a1)(std::ios_base::event, std::ios_base &, int), int a2) {
  return a0.register_callback(a1, a2);
}

bool f24(bool a0) { return std::ios_base::sync_with_stdio(a0); }

std::ios_base::iostate f25(const std::ios_base &a0) { return a0.rdstate(); }

void f26(std::ios_base &a0, std::ios_base::iostate a1) { return a0.clear(a1); }

void f27(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.setstate(a1);
}

bool f28(const std::ios_base &a0) { return a0.good(); }

bool f29(const std::ios_base &a0) { return a0.eof(); }

bool f30(const std::ios_base &a0) { return a0.fail(); }

bool f31(const std::ios_base &a0) { return a0.bad(); }

std::ios_base::iostate f32(const std::ios_base &a0) { return a0.exceptions(); }

void f33(std::ios_base &a0, std::ios_base::iostate a1) {
  return a0.exceptions(a1);
}

std::ios_base::fmtflags f34() { return std::ios_base::boolalpha; }

std::ios_base::fmtflags f35() { return std::ios_base::dec; }

std::ios_base::fmtflags f36() { return std::ios_base::fixed; }

std::ios_base::fmtflags f37() { return std::ios_base::hex; }

std::ios_base::fmtflags f38() { return std::ios_base::internal; }

std::ios_base::fmtflags f39() { return std::ios_base::left; }

std::ios_base::fmtflags f40() { return std::ios_base::oct; }

std::ios_base::fmtflags f41() { return std::ios_base::right; }

std::ios_base::fmtflags f42() { return std::ios_base::scientific; }

std::ios_base::fmtflags f43() { return std::ios_base::showbase; }

std::ios_base::fmtflags f44() { return std::ios_base::showpoint; }

std::ios_base::fmtflags f45() { return std::ios_base::showpos; }

std::ios_base::fmtflags f46() { return std::ios_base::skipws; }

std::ios_base::fmtflags f47() { return std::ios_base::unitbuf; }

std::ios_base::fmtflags f48() { return std::ios_base::uppercase; }

std::ios_base::fmtflags f49() { return std::ios_base::adjustfield; }

std::ios_base::fmtflags f50() { return std::ios_base::basefield; }

std::ios_base::fmtflags f51() { return std::ios_base::floatfield; }

std::ios_base::iostate f52() { return std::ios_base::badbit; }

std::ios_base::iostate f53() { return std::ios_base::eofbit; }

std::ios_base::iostate f54() { return std::ios_base::failbit; }

std::ios_base::iostate f55() { return std::ios_base::goodbit; }

std::ios_base::openmode f56() { return std::ios_base::app; }

std::ios_base::openmode f57() { return std::ios_base::ate; }

std::ios_base::openmode f58() { return std::ios_base::binary; }

std::ios_base::openmode f59() { return std::ios_base::in; }

std::ios_base::openmode f60() { return std::ios_base::out; }

std::ios_base::openmode f61() { return std::ios_base::trunc; }

const std::error_category &f62() { return std::iostream_category(); }

std::ios_base &f63(std::ios_base &a0) { return std::left(a0); }

std::error_code f64(std::io_errc a0) { return std::make_error_code(a0); }

std::error_condition f65(std::io_errc a0) {
  return std::make_error_condition(a0);
}

std::ios_base &f66(std::ios_base &a0) { return std::noboolalpha(a0); }

std::ios_base &f67(std::ios_base &a0) { return std::noshowbase(a0); }

std::ios_base &f68(std::ios_base &a0) { return std::noshowpoint(a0); }

std::ios_base &f69(std::ios_base &a0) { return std::noshowpos(a0); }

std::ios_base &f70(std::ios_base &a0) { return std::noskipws(a0); }

std::ios_base &f71(std::ios_base &a0) { return std::nounitbuf(a0); }

std::ios_base &f72(std::ios_base &a0) { return std::nouppercase(a0); }

std::ios_base &f73(std::ios_base &a0) { return std::oct(a0); }

std::ios_base &f74(std::ios_base &a0) { return std::right(a0); }

std::ios_base &f75(std::ios_base &a0) { return std::scientific(a0); }

std::ios_base &f76(std::ios_base &a0) { return std::showbase(a0); }

std::ios_base &f77(std::ios_base &a0) { return std::showpoint(a0); }

std::ios_base &f78(std::ios_base &a0) { return std::showpos(a0); }

std::ios_base &f79(std::ios_base &a0) { return std::skipws(a0); }

using t12 = std::streamoff;

using t13 = std::streamsize;

std::ios_base &f80(std::ios_base &a0) { return std::unitbuf(a0); }

std::ios_base &f81(std::ios_base &a0) { return std::uppercase(a0); }

using t14 = std::wios;
