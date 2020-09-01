use {
    std::string::ToString,
    ::petgraph::{
        EdgeType,
        graphmap::{
            GraphMap,
            NodeTrait
        }
    },
    super::*
};

impl<N: NodeTrait + ToString, E, Ty: EdgeType> DebugVis for GraphMap<N, E, Ty> {
    fn debug_visualize(&self) -> DebugVisJSON {
        let nodes = self.nodes()
            .map(|id| GraphNode::new(id))
            .collect();

        let edges = self.all_edges()
            .map(|(from, to, _)| GraphEdge::new(from, to))
            .collect();

        DebugVisJSON::Graph {
            nodes,
            edges
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        petgraph::graphmap::UnGraphMap,
        crate::{DebugVis, raw_dbg_json},
    };

    #[test]
    fn test_petgraph_json() {
        let mut g = UnGraphMap::new();

        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        g.add_node(4);
        g.add_node(5);

        g.add_edge(0, 1, ());
        g.add_edge(0, 2, ());
        g.add_edge(2, 4, ());
        g.add_edge(2, 5, ());

        let value = serde_json::to_string(&raw_dbg_json(g.debug_visualize())).unwrap();
        let expected = r#"{
            "kind":{"graph":true},
            "nodes":[{"id":"0"},{"id":"1"},{"id":"2"},{"id":"3"},{"id":"4"},{"id":"5"}],
            "edges":[
                {"from":"0","to":"1"},{"from":"0","to":"2"},
                {"from":"2","to":"4"},{"from":"2","to":"5"}
            ]
        }"#.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        assert_eq!(value, expected);
    }
}

