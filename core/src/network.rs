//! Path networks for road vehicles, taxiing aircraft and runways (ADR-020).
//!
//! Plain data structures in the world state: nodes sit on grid cells, edges
//! are directed. Traffic rules, lanes, right of way and reservations belong
//! to the module "Verkehr & Verkehrssteuerung" (roadmap step 3.4).
// TODO(fahrplan-ref: 3.4): lane properties, right-of-way rules and
// reservations of sections on top of these structures.

use crate::geometry::CellPos;
use crate::hash::StateHasher;

/// Kind of network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkKind {
    /// Roads with lanes for vehicles.
    Road,
    /// Taxiways with taxi nodes drawn by the player.
    Taxiway,
    /// Runways.
    Runway,
}

impl NetworkKind {
    /// All kinds in storage order.
    pub const ALL: [Self; 3] = [Self::Road, Self::Taxiway, Self::Runway];

    /// Position of the kind in [`NetworkKind::ALL`].
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::Road => 0,
            Self::Taxiway => 1,
            Self::Runway => 2,
        }
    }
}

/// Identifier of a node, valid within its network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

/// Why an edge could not be added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    /// A node of the edge does not exist.
    UnknownNode,
    /// Start and end of the edge are the same node.
    SelfLoop,
    /// The edge exists already.
    Duplicate,
}

/// A directed graph on grid cells.
#[derive(Debug, Clone)]
pub struct Network {
    kind: NetworkKind,
    nodes: Vec<CellPos>,
    edges: Vec<(NodeId, NodeId)>,
}

impl Network {
    /// Empty network of the given kind.
    #[must_use]
    pub fn new(kind: NetworkKind) -> Self {
        Self {
            kind,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Kind of the network.
    #[must_use]
    pub fn kind(&self) -> NetworkKind {
        self.kind
    }

    /// Adds a node on a cell; an existing node on that cell is reused.
    pub fn add_node(&mut self, pos: CellPos) -> NodeId {
        if let Some(index) = self.nodes.iter().position(|n| *n == pos) {
            return NodeId(u32::try_from(index).unwrap_or(u32::MAX));
        }
        self.nodes.push(pos);
        NodeId(u32::try_from(self.nodes.len() - 1).unwrap_or(u32::MAX))
    }

    /// Cell of a node.
    #[must_use]
    pub fn position(&self, node: NodeId) -> Option<CellPos> {
        self.nodes.get(node.0 as usize).copied()
    }

    /// Adds a directed edge.
    ///
    /// # Errors
    ///
    /// Fails if a node is unknown, both nodes are the same or the edge
    /// exists already.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<(), NetworkError> {
        if self.position(from).is_none() || self.position(to).is_none() {
            return Err(NetworkError::UnknownNode);
        }
        if from == to {
            return Err(NetworkError::SelfLoop);
        }
        if self.edges.contains(&(from, to)) {
            return Err(NetworkError::Duplicate);
        }
        self.edges.push((from, to));
        Ok(())
    }

    /// Nodes reachable by one edge from `node`, in the order the edges were
    /// added.
    pub fn successors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.edges
            .iter()
            .filter(move |(from, _)| *from == node)
            .map(|(_, to)| *to)
    }

    /// Number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Cells of the nodes, indexed by node id.
    #[must_use]
    pub fn nodes(&self) -> &[CellPos] {
        &self.nodes
    }

    /// Directed edges in the order they were added.
    #[must_use]
    pub fn edges(&self) -> &[(NodeId, NodeId)] {
        &self.edges
    }

    /// Network of a save game; `None` if an edge refers to an unknown node,
    /// is a loop or appears twice. Positions are checked by the caller.
    pub(crate) fn from_parts(
        kind: NetworkKind,
        nodes: Vec<CellPos>,
        edges: Vec<(NodeId, NodeId)>,
    ) -> Option<Self> {
        let count = nodes.len();
        let mut seen = std::collections::HashSet::with_capacity(edges.len());
        let valid = edges.iter().all(|&(a, b)| {
            (a.0 as usize) < count && (b.0 as usize) < count && a != b && seen.insert((a, b))
        });
        valid.then_some(Self { kind, nodes, edges })
    }

    /// Feeds the network into `hasher`.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        hasher.u32(self.nodes.len() as u32);
        for n in &self.nodes {
            hasher.u32(u32::from(n.x));
            hasher.u32(u32::from(n.y));
            hasher.bytes(&n.floor.to_le_bytes());
        }
        hasher.u32(self.edges.len() as u32);
        for (a, b) in &self.edges {
            hasher.u32(a.0);
            hasher.u32(b.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: u16, y: u16) -> CellPos {
        CellPos { x, y, floor: 0 }
    }

    #[test]
    fn nodes_are_unique_per_cell() {
        let mut net = Network::new(NetworkKind::Taxiway);
        let a = net.add_node(pos(1, 1));
        let b = net.add_node(pos(2, 1));
        assert_ne!(a, b);
        assert_eq!(net.add_node(pos(1, 1)), a);
        assert_eq!(net.node_count(), 2);
        assert_eq!(net.position(b), Some(pos(2, 1)));
        assert_eq!(net.position(NodeId(9)), None);
        assert_eq!(net.kind(), NetworkKind::Taxiway);
    }

    #[test]
    fn edges_are_directed_and_validated() {
        let mut net = Network::new(NetworkKind::Road);
        let a = net.add_node(pos(0, 0));
        let b = net.add_node(pos(5, 0));
        let c = net.add_node(pos(9, 0));
        net.add_edge(a, b).unwrap();
        net.add_edge(a, c).unwrap();
        net.add_edge(b, a).unwrap();
        assert_eq!(net.add_edge(a, b), Err(NetworkError::Duplicate));
        assert_eq!(net.add_edge(a, a), Err(NetworkError::SelfLoop));
        assert_eq!(net.add_edge(a, NodeId(7)), Err(NetworkError::UnknownNode));
        assert_eq!(net.add_edge(NodeId(7), a), Err(NetworkError::UnknownNode));
        assert_eq!(net.successors(a).collect::<Vec<_>>(), vec![b, c]);
        assert_eq!(net.successors(c).count(), 0);
        assert_eq!(net.edge_count(), 3);
    }

    #[test]
    fn kinds_have_distinct_storage_indices() {
        let indices: Vec<usize> = NetworkKind::ALL.iter().map(|k| k.index()).collect();
        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn hash_reflects_nodes_and_edges() {
        let hash = |net: &Network| {
            let mut h = StateHasher::new();
            net.hash_into(&mut h);
            h.finish()
        };
        let mut a = Network::new(NetworkKind::Runway);
        let b = Network::new(NetworkKind::Runway);
        assert_eq!(hash(&a), hash(&b));
        let n1 = a.add_node(pos(0, 0));
        let n2 = a.add_node(pos(1, 0));
        let without_edge = hash(&a);
        a.add_edge(n1, n2).unwrap();
        assert_ne!(hash(&a), without_edge);
    }
}
