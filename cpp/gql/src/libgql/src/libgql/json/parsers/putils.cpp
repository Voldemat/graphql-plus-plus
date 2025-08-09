#include "./putils.hpp"

#include <format>
#include <functional>
#include <string>

#include "../utils.hpp"
#include "./shared.hpp"
#include "utils.hpp"

namespace gql::json::parsers::utils {
void assert_has_member(const JSONValue &value, const char *key,
                       const std::string &path) {
    if (!value.HasMember(key)) {
        throw shared::ParsingError(std::format("{}.{} is not found", path, key),
                                   shared::ParsingErrorType::NO_MEMBER);
    }
};

void assert_member_is_valid(const std::string &key, const bool &isValid,
                            const std::string &path) {
    if (!isValid) {
        throw shared::ParsingError(std::format("{}.{} is invalid", path, key),
                                   shared::ParsingErrorType::INVALID_MEMBER);
    };
};

unsigned int extractUint(const JSONValue &value, const char *key,
                         const std::string &path) {
    return json::parsers::utils::extractValue(
        value, key, path, [](const JSONValue &v) -> bool { return v.IsUint(); },
        ::gql::utils::FuncFromLambda(
            [](const JSONValue &v) -> unsigned int { return v.GetUint(); }));
};
};  // namespace gql::json::parsers::utils
