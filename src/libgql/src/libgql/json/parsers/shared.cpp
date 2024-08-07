#include "./shared.hpp"

#include <string>

json::parsers::shared::ParsingError::ParsingError(
    const std::string &message,
    const json::parsers::shared::ParsingErrorType &errorType)
    : message{ message }, errorType{ errorType } {};
const char *json::parsers::shared::ParsingError::what() const noexcept {
    return message.c_str();
};
