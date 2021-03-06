use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree};

pub struct GenerateKeyword;

impl Rule for GenerateKeyword {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::GenerateRegion(_) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_keyword")
    }

    fn hint(&self) -> String {
        String::from("`generate`/`endgenerate` must be omitted")
    }

    fn reason(&self) -> String {
        String::from("")
    }
}
