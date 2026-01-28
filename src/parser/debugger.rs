use super::*;

pub struct Debugger;

impl Debugger {
    pub fn print_nodes_tree(node: &Node, parser: &Parser) {
        Self::nodes_tree_rec(node, parser, "".to_string(), true, true)
    }

    fn nodes_tree_rec(node: &Node, parser: &Parser, prefix: String, is_last_sibling: bool, is_root: bool) {
        let connector = if is_root {
            ""
        } else if is_last_sibling {
            "└── "
        } else {
            "├── "
        };

        let tokens = &parser.tks[node.range.clone()].iter()
            .flat_map(|t| parser.get_src(t.range.clone()).iter().copied().chain(std::iter::once(b' ')))
            .collect::<Vec<u8>>();
        let kind = &node.kind;
        println!("{}{}{:?} -> {}", prefix, connector, kind, String::from_utf8_lossy(tokens.as_slice()));

        let child_prefix = prefix +
            if is_root {
                ""
            } else if is_last_sibling {
                "    "
            } else {
                "│   "
            };

        let mut children_to_visit: Vec<&Node> = Vec::new();
        node.visit_children(|child_node| {
            children_to_visit.push(child_node);
        });

        let num_children = children_to_visit.len();
        for (i, child_node) in children_to_visit.into_iter().enumerate() {
            Self::nodes_tree_rec(child_node, parser, child_prefix.clone(), i == num_children - 1, false);
        }
    }
}