use crate::data_types::entities::{ResearchGraph, TechEdge};

pub(crate) fn build_research_graph(edges: &[TechEdge]) -> ResearchGraph {
    let mut graph = ResearchGraph::default();
    for edge in edges {
        graph
            .prereqs
            .entry(edge.to.clone())
            .or_default()
            .push(edge.from.clone());
        graph
            .unlocks
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
    }
    graph
}
