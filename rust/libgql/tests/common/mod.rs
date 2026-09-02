pub mod builders;

pub fn hir_nodes_to_string(
    hir_to_lir_config: &impl codeform::hir_to_lir::config::Config,
    lir_printer_config: &impl codeform::lir_printer::Config,
    hir_nodes: codeform::ir::hir::builders::NodesVec<'_>,
) -> String {
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
        hir_to_lir_config,
        &mut hir_to_lir_state,
        hir_nodes,
    );
    let mut io_writer = Vec::<u8>::new();
    let mut lir_printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut io_writer,
        lir_printer_config,
        &mut lir_printer_state,
        &lir_nodes,
    )
    .unwrap();
    String::from_utf8(io_writer).unwrap()
}
