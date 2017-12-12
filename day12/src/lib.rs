pub mod parser;
pub use parser::parse_connections;

type Connections = (usize, Vec<usize>);
type Graph = Vec<bool>;

fn map_connections(graph: &mut Graph, connections: &[Connections], node: usize) {
    map_connections_generic(graph, connections, node, true, &|b: bool| !b);
}

fn map_connections_node(
    graph: &mut Vec<Node>,
    connections: &[Connections],
    node: usize,
    group: usize,
) {
    map_connections_generic(graph, connections, node, Some(group), &|n| n.is_none());
}

fn map_connections_generic<N, IsFalse>(
    graph: &mut Vec<N>,
    connections: &[Connections],
    node: usize,
    true_value: N,
    is_false: &IsFalse,
) where
    N: Copy,
    IsFalse: Fn(N) -> bool,
{
    graph[node] = true_value;
    for &connection in connections[node].1.iter() {
        if is_false(graph[connection]) {
            map_connections_generic(graph, connections, connection, true_value, is_false);
        }
    }
}

pub fn connected_to_zero(connections: &[Connections]) -> Vec<usize> {
    assert!(
        connections.iter().enumerate().all(
            |(idx, cxn)| idx == cxn.0,
        ),
        "Input connections must be sorted by node index"
    );
    let mut graph = vec![false; connections.len()];
    if graph.len() > 0 {
        map_connections(&mut graph, connections, 0)
    }
    graph
        .iter()
        .enumerate()
        .filter(|&(_, node)| *node)
        .map(|(index, _)| index)
        .collect()
}

type Node = Option<usize>;

pub fn count_groups(connections: &[Connections]) -> usize {
    assert!(
        connections.iter().enumerate().all(
            |(idx, cxn)| idx == cxn.0,
        ),
        "Input connections must be sorted by node index"
    );
    let mut graph: Vec<Node> = vec![None; connections.len()];
    let mut group = 0;

    while let Some(idx) = graph
        .iter()
        .enumerate()
        .filter(|&(_, g)| g.is_none())
        .map(|(idx, _)| idx)
        .next()
    {
        group += 1;
        map_connections_node(&mut graph, connections, idx, group);
    }

    group
}
