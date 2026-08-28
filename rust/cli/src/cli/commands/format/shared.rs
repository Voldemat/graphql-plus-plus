pub fn print_lir_nodes<TWriter: std::io::Write>(
    writer: &mut TWriter,
    config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node<'_>>,
) -> std::io::Result<()> {
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        writer,
        &config,
        &mut printer_state,
        &lir_nodes,
    )
}

pub fn format_print_action(
    graphql_path: &std::path::Path,
    lir_printer_config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node>,
) -> Result<(), String> {
    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(graphql_path).unwrap());
    print_lir_nodes(&mut writer, lir_printer_config, lir_nodes).unwrap();
    Ok(())
}

fn format_check_action(
    graphql_path: &std::path::Path,
    initial_buffer: &str,
    lir_printer_config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node>,
) -> Result<(), String> {
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    print_lir_nodes(&mut writer, lir_printer_config, lir_nodes).unwrap();
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    match super::text_diff::get_diff_string(initial_buffer, &formatted_string) {
        None => Ok(()),
        Some(diff_string) => Err(format!(
            "{}\n{}",
            console::style(format!("{}:", graphql_path.to_string_lossy()))
                .blue(),
            diff_string
        )),
    }
}

pub fn format_lir_nodes_action(
    is_check: bool,
    graphql_path: &std::path::Path,
    initial_buffer: &str,
    lir_printer_config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node>,
) -> Result<(), String> {
    if is_check {
        format_check_action(
            graphql_path,
            initial_buffer,
            lir_printer_config,
            lir_nodes,
        )
    } else {
        format_print_action(graphql_path, lir_printer_config, lir_nodes)
    }
}

pub trait ASTNodeWrapper {
    type ASTNode<'buffer>;
}

pub struct ClientASTNodeWrapper {}

impl ASTNodeWrapper for ClientASTNodeWrapper {
    type ASTNode<'buffer> =
        libgql::parsers::file::client::ast::ASTNode<'buffer>;
}

pub struct ServerASTNodeWrapper {}

impl ASTNodeWrapper for ServerASTNodeWrapper {
    type ASTNode<'buffer> =
        libgql::parsers::file::server::ast::ASTNode<'buffer>;
}

pub fn format_config<
    TASTNodeWrapper: ASTNodeWrapper,
    TBufferToASTNodes: for<'buffer> Fn(
        &std::path::PathBuf,
        &'buffer str,
    )
        -> Result<Vec<TASTNodeWrapper::ASTNode<'buffer>>, String>,
    TASTNodesToHIRNodes: for<'buffer> Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    graphql_paths: &[std::path::PathBuf],
    buffer_to_ast_nodes: TBufferToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    hir_to_lir_config: &codeform::hir_to_lir::config::Config,
    lir_printer_config: &codeform::lir_printer::Config,
    is_check: bool,
) -> Vec<String> {
    graphql_paths
        .into_iter()
        .filter_map(|graphql_path| -> Option<String> {
            let buffer = std::fs::read_to_string(&graphql_path).unwrap();
            let ast_nodes = match buffer_to_ast_nodes(&graphql_path, &buffer) {
                Ok(nodes) => nodes,
                Err(error) => {
                    return Some(error);
                }
            };
            let hir_nodes = ast_nodes_to_hir_nodes(&buffer, ast_nodes);
            let mut hir_to_lir_state =
                codeform::hir_to_lir::state::State::default();
            let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
                hir_to_lir_config,
                &mut hir_to_lir_state,
                hir_nodes,
            );
            super::shared::format_lir_nodes_action(
                is_check,
                &graphql_path,
                &buffer,
                lir_printer_config,
                lir_nodes,
            )
            .err()
        })
        .collect::<Vec<_>>()
}
