#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

std::ios_base::fmtflags f89(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator&(a0, a1);
}

std::ios_base::iostate f90(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator&(a0, a1);
}

std::ios_base::openmode f91(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator&(a0, a1);
}

const std::ios_base::fmtflags &f92(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::iostate &f93(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::openmode &f94(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator&=(a0, a1);
}

std::ios_base::fmtflags f95(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator^(a0, a1);
}

std::ios_base::iostate f96(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator^(a0, a1);
}

std::ios_base::openmode f97(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator^(a0, a1);
}

const std::ios_base::fmtflags &f98(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::iostate &f99(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::openmode &f100(std::ios_base::openmode &a0,
                                    std::ios_base::openmode a1) {
  return std::operator^=(a0, a1);
}

std::ios_base::fmtflags f101(std::ios_base::fmtflags a0,
                             std::ios_base::fmtflags a1) {
  return std::operator|(a0, a1);
}

std::ios_base::iostate f102(std::ios_base::iostate a0,
                            std::ios_base::iostate a1) {
  return std::operator|(a0, a1);
}

std::ios_base::openmode f103(std::ios_base::openmode a0,
                             std::ios_base::openmode a1) {
  return std::operator|(a0, a1);
}

const std::ios_base::fmtflags &f104(std::ios_base::fmtflags &a0,
                                    std::ios_base::fmtflags a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::iostate &f105(std::ios_base::iostate &a0,
                                   std::ios_base::iostate a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::openmode &f106(std::ios_base::openmode &a0,
                                    std::ios_base::openmode a1) {
  return std::operator|=(a0, a1);
}

std::ios_base::fmtflags f107(std::ios_base::fmtflags a0) {
  return std::operator~(a0);
}

std::ios_base::iostate f108(std::ios_base::iostate a0) {
  return std::operator~(a0);
}

std::ios_base::openmode f109(std::ios_base::openmode a0) {
  return std::operator~(a0);
}
