use cairo_lang_semantic::ConcreteFunctionWithBodyId;
use cairo_lang_utils::graph_algos::graph_node::GraphNode;
use cairo_lang_utils::graph_algos::strongly_connected_components::ComputeScc;

use super::strongly_connected_components::concrete_function_with_body_scc;
use crate::db::LoweringGroup;

/// A node to use in graph-algorithms.
#[derive(Clone)]
pub struct ConcreteFunctionWithBodyNode<'a> {
    pub function_id: ConcreteFunctionWithBodyId,
    pub db: &'a dyn LoweringGroup,
}
impl<'a> GraphNode for ConcreteFunctionWithBodyNode<'a> {
    type NodeId = ConcreteFunctionWithBodyId;

    fn get_neighbors(&self) -> Vec<Self> {
        let Ok(direct_callees) = self.db.concrete_function_with_body_lowered_direct_callees(self.function_id)
            else { return vec![] };
        direct_callees
            .into_iter()
            .map(|callee| ConcreteFunctionWithBodyNode { function_id: callee, db: self.db })
            .collect()
    }

    fn get_id(&self) -> Self::NodeId {
        self.function_id
    }
}
impl<'a> ComputeScc for ConcreteFunctionWithBodyNode<'a> {
    fn compute_scc(&self) -> Vec<Self::NodeId> {
        concrete_function_with_body_scc(self.db, self.function_id)
    }
}