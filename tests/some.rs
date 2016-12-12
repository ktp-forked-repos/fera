#[macro_use]
extern crate fera_graphs as graphs;

use graphs::prelude::*;
use graphs::components::Components;
use graphs::cycles::Cycles;
use graphs::trees::Trees;

struct Case {
    g: StaticGraph,
    is_connected: bool,
    is_acyclic: bool,
    is_tree: bool,
}

fn cases() -> Vec<Case> {
    vec![
        Case { // 0
            g: graph!(StaticGraph),
            is_connected: true,
            is_acyclic: true,
            is_tree: true,
        },
        Case { // 1
            g: graph!(StaticGraph, 1),
            is_connected: true,
            is_acyclic: true,
            is_tree: true,
        },
        Case { // 2
            g: graph!(StaticGraph, 2),
            is_connected: false,
            is_acyclic: true,
            is_tree: false,
        },
        Case { // 3
            g: graph!(StaticGraph, 2, (0, 1)),
            is_connected: true,
            is_acyclic: true,
            is_tree: true,
        },
        Case { // 4
            g: graph!(StaticGraph, 3, (2, 1)),
            is_connected: false,
            is_acyclic: true,
            is_tree: false,
        },
        Case { // 5
            g: graph!(StaticGraph, 3, (2, 1)),
            is_connected: false,
            is_acyclic: true,
            is_tree: false,
        },
        Case { // 6
            g: graph!(StaticGraph, 3, (0, 1), (1, 2)),
            is_connected: true,
            is_acyclic: true,
            is_tree: true,
        },
        Case { // 7
            g: graph!(StaticGraph, 3, (0, 1), (0, 2), (1, 2)),
            is_connected: true,
            is_acyclic: false,
            is_tree: false,
        },
        Case { // 8
            g: graph!(StaticGraph, 4, (0, 1), (0, 2)),
            is_connected: false,
            is_acyclic: true,
            is_tree: false,
        },
        Case { // 9
            g: graph!(StaticGraph, 4, (1, 2), (2, 3), (3, 1)),
            is_connected: false,
            is_acyclic: false,
            is_tree: false,
        },
    ]
}

#[test]
fn is_connected() {
    for (i, case) in cases().iter().enumerate() {
        assert!(case.is_connected == case.g.is_connected(),
                format!("Case {}", i));
    }
}

#[test]
fn is_acyclic() {
    for (i, case) in cases().iter().enumerate() {
        assert!(case.is_acyclic == case.g.is_acyclic(),
                format!("Case {}", i));
    }
}

#[test]
fn is_tree() {
    for (i, case) in cases().iter().enumerate() {
        assert!(case.is_tree == case.g.is_tree(), format!("Case {}", i));
    }
}