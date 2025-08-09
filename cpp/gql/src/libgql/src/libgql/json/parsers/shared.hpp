#pragma once

#include <exception>
#include <string>

namespace gql::json::parsers::shared {
enum class ParsingErrorType {
    NO_MEMBER,
    INVALID_MEMBER,
    INVALID_TYPE
};

class ParsingError : public std::exception {
public:
    std::string message;
    ParsingErrorType errorType;
    explicit ParsingError(const std::string &message,
                          const ParsingErrorType &errorType);
    const char *what() const noexcept;
};
};
