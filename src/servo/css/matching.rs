// High-level interface to CSS selector matching.

use css::node_util::NodeUtil;
use css::select_handler::NodeSelectHandler;
use dom::node::AbstractNode;
use newcss::complete::CompleteSelectResults;
use newcss::select::{SelectCtx, SelectResults};

pub trait MatchMethods {
    fn restyle_subtree(&self, select_ctx: &SelectCtx);
}

impl MatchMethods for AbstractNode {
    /**
     * Performs CSS selector matching on a subtree.
     *
     * This is, importantly, the function that updates the layout data for
     * the node (the reader-auxiliary box in the COW model) with the
     * computed style.
     */
    fn restyle_subtree(&self, select_ctx: &SelectCtx) {
        // Only elements have styles
        if self.is_element() {
            let select_handler = NodeSelectHandler { node: *self };
            let incomplete_results = select_ctx.select_style(self, &select_handler);
            // Combine this node's results with its parent's to resolve all inherited values
            let complete_results = compose_results(*self, incomplete_results);
            self.set_css_select_results(complete_results);
        }

        for self.each_child |kid| {
            kid.restyle_subtree(select_ctx); 
        }
    }
}

fn compose_results(node: AbstractNode, results: SelectResults) -> CompleteSelectResults {
    match find_parent_element_node(node) {
        None => CompleteSelectResults::new_root(results),
        Some(parent_node) => {
            let parent_results = parent_node.get_css_select_results();
            CompleteSelectResults::new_from_parent(parent_results, results)
        }
    }    
}

fn find_parent_element_node(node: AbstractNode) -> Option<AbstractNode> {
    match node.parent_node() {
        Some(parent) if parent.is_element() => Some(parent),
        Some(parent) => find_parent_element_node(parent),
        None => None,
    }
}

