pub trait Config {
    fn get_indent_width(self: &Self) -> codeform::ir::shared::IndentWidth;
}
