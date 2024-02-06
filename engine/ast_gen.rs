use std::fs;
use std::io::Read;
use tree_sitter::{Node, Parser};

pub fn grab_ast(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>  {
    let mut parser = Parser::new();
    let language = tree_sitter_c::language();
    parser.set_language(language).expect("Error loading C grammar");

    let mut source_code = String::new();
    let mut file = fs::File::open(file)?;
    file.read_to_string(&mut source_code)?;

    let parsed_tree = parser.parse(&source_code, None).expect("Error parsing code");
    let root_node = parsed_tree.root_node();
    let mut ast_nodes = Vec::new();

    fn collect_nodes(node: Node, source_code: &str, ast_nodes: &mut Vec<String>, level: usize) {
        let indent = " ".repeat(level * 2);
        let kind = node.kind();
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let text = &source_code[start_byte..end_byte];

        let node_text = format!("{}{}: {}", indent, kind, text);
        ast_nodes.push(node_text);

        for child in node.children(&mut node.walk()) {
            collect_nodes(child, source_code, ast_nodes, level + 1);
        }
    }
    collect_nodes(root_node, &source_code, &mut ast_nodes, 0);
    Ok(ast_nodes)
}
