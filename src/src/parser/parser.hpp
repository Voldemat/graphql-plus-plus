#include <streambuf>
#include <string>

#include "src/lexer/lexer.hpp"

class Identifier {
    std::string name;
};

class Parser {
public:
    std::basic_streambuf<GQL_TOKEN*>* stream;
    Parser(std::basic_streambuf<GQL_TOKEN *>* tokens): stream{tokens} {};
    getAstTree();
};
