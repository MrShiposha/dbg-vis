use serde::Serialize;
pub extern crate serde_json;

mod graph;

#[cfg(feature = "with-petgraph")]
mod petgraph;

use graph::*;

#[macro_export]
#[cfg(feature = "dbg-vis-macro")]
macro_rules! dbg_vis {
    (let $dbg_watch:ident: { $($watch_point:ident),+ }) => {
        #[allow(unused)]
        #[allow(unused_mut)]
        let mut $dbg_watch;

        #[cfg(debug_assertions)]
        {
            $dbg_watch = {
                use $crate::DebugVis;

                #[allow(non_camel_case_types)]
                #[derive(Default)]
                pub struct ___DBG_VIS_STRUCT___ {
                    #[allow(unused)]
                    $(
                        $watch_point: $crate::JSON
                    ),+
                }

                ___DBG_VIS_STRUCT___::default()
            };
        }

        #[cfg(not(debug_assertions))]
        {
            $dbg_watch = {
                #[allow(non_camel_case_types)]
                pub struct ___DBG_VIS_STRUCT___;

                ___DBG_VIS_STRUCT___
            };
        }
    };

    ($dbg_watch:ident.$watch_point:ident = $value:expr) => {
        #[cfg(debug_assertions)]
        {
            $dbg_watch.$watch_point = $crate::serde_json::to_string(
                &$crate::raw_dbg_json(($value).debug_visualize())
            ).unwrap();
        }
    };

    ($dbg_watch:ident.$watch_point:ident) => {
        dbg_vis![$dbg_watch.$watch_point = $watch_point];
    };
}

#[macro_export]
#[cfg(not(feature = "dbg-vis-macro"))]
macro_rules! dbg_vis {
    ($($tt:tt)*) => {}
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
