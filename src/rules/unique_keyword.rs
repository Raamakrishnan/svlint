use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree, UniquePriority};

pub struct UniqueKeyword;

impl Rule for UniqueKeyword {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::UniquePriority(UniquePriority::Unique(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("unique_keyword")
    }

    fn hint(&self) -> String {
        String::from("`unique` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this causes mismatch between simulaton and synthesis")
    }
}
