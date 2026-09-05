pub trait Error<'buffer>: std::fmt::Display + std::fmt::Debug {
    fn get_location(
        self: &Self,
    ) -> &crate::parsers::file::shared::ast::NodeLocation<'buffer>;
}
