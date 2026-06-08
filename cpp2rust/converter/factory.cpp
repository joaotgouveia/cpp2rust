// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/factory.h"

#include "converter/mapper.h"
#include "converter/models/converter_refcount.h"

namespace cpp2rust {

std::unique_ptr<Converter>
CreateConverter(std::string &rs_code, clang::ASTContext &ctx, Model model,
                const std::vector<std::string_view> &rule_dirs,
                bool allow_partial_tgts) {
  Mapper::LoadTranslationRules(model, ctx, rule_dirs, allow_partial_tgts);
  switch (model) {
  case Model::kUnsafe:
    return std::make_unique<Converter>(rs_code, ctx);
  case Model::kRefCount:
    return std::make_unique<ConverterRefCount>(rs_code, ctx);
  }
  std::unreachable();
}

} // namespace cpp2rust
