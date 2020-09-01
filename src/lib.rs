use serde::Serialize;
pub extern crate serde_json;

mod graph;

#[cfg(feature = "with_petgraph")]
mod petgraph;

use graph::*;

#[macro_export]
macro_rules! dbg_vis {
    ($point_name:ident: $watch:expr) => {

        let _dbg_vis;

        #[cfg(debug_assertions)]
        {
            _dbg_vis = {
                #[allow(non_camel_case_types)]
                pub struct ___DBG_VIS_STRUCT___ {
                    #[allow(unused)]
                    $point_name: $crate::JSON
                }

                ___DBG_VIS_STRUCT___ {
                    $point_name: $crate::serde_json::to_string(
                        &$crate::raw_dbg_json(($watch).debug_visualize())
                    ).unwrap()
                }
            };
        }

        #[cfg(not(debug_assertions))]
        {
            _dbg_vis = {
                #[allow(non_camel_case_types)]
                pub struct ___DBG_VIS_STRUCT___;

                ___DBG_VIS_STRUCT___
            };
        }
    };
}

pub type JSON = String;

pub trait DebugVis {
    fn debug_visualize(&self) -> DebugVisJSON;
}

/// https://hediet.github.io/visualization/docs/visualization-data-schema.json
pub enum DebugVisJSON {
    Graph {
        nodes: Vec<GraphNode>,
        edges: Vec<GraphEdge>,
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum RawDebugVisJSON {
    Graph {
        kind: GraphKind,
        nodes: Vec<GraphNode>,
        edges: Vec<GraphEdge>,
    }
}

pub fn raw_dbg_json(dbg_vis: DebugVisJSON) -> RawDebugVisJSON {
    match dbg_vis {
        DebugVisJSON::Graph {
            nodes,
            edges
        } => RawDebugVisJSON::Graph {
            kind: kind::graph(),
            nodes,
            edges
        }
    }
}

mod kind {
    use crate::graph::*;

    pub fn graph() -> GraphKind {
        GraphKind::new()
    }
}

pub mod prelude {
    pub use crate::{
        DebugVis,
        graph::*,
    };

    #[cfg(feature = "with_petgraph")]
    pub use petgraph::*;
}
