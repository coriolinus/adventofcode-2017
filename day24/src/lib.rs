use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Component {
    left: usize,
    right: usize,
}

impl Ord for Component {
    fn cmp(&self, other: &Component) -> Ordering {
        match (self.left + self.right).cmp(&(other.left + other.right)) {
            Ordering::Equal => self.left.cmp(&other.left),
            otherwise => otherwise,
        }
    }
}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Component) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Component {
    pub fn new(left: usize, right: usize) -> Component {
        Component {
            left: left,
            right: right,
        }
    }

    pub fn other(&self, v: usize) -> Option<usize> {
        if v == self.left {
            Some(self.right)
        } else if v == self.right {
            Some(self.left)
        } else {
            None
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.left, self.right)
    }
}

pub struct ComponentBin {
    bin: HashMap<usize, BTreeSet<Component>>,
}

impl ComponentBin {
    pub fn new() -> ComponentBin {
        ComponentBin {
            bin: HashMap::new(),
        }
    }

    pub fn insert(&mut self, component: Component) {
        self.bin
            .entry(component.left)
            .or_insert(BTreeSet::new())
            .insert(component);
        self.bin
            .entry(component.right)
            .or_insert(BTreeSet::new())
            .insert(component);
    }

    fn remove(&mut self, component: Component) -> bool {
        let c_l = self.bin
            .entry(component.left)
            .or_insert(BTreeSet::new())
            .remove(&component);
        let c_r = self.bin
            .entry(component.right)
            .or_insert(BTreeSet::new())
            .remove(&component);
        c_l && c_r || (c_l && component.left == component.right)
    }

    /// find the nth component, one of whose values is `side`, and
    /// remove and return the `index` biggest one.
    fn pop_nth(&mut self, side: usize, index: usize) -> Option<Component> {
        let nth_biggest = {
            let set = self.bin.entry(side).or_insert(BTreeSet::new());
            set.iter().rev().skip(index).cloned().next()
        };

        if let Some(component) = nth_biggest {
            if !self.remove(component) {
                panic!("nth biggest not present where expected");
            }
        }
        nth_biggest
    }

    fn size_len(&self, side: usize) -> Option<usize> {
        self.bin.get(&side).map(|s| s.len())
    }
}

impl Deref for ComponentBin {
    type Target = HashMap<usize, BTreeSet<Component>>;
    fn deref(&self) -> &HashMap<usize, BTreeSet<Component>> {
        &self.bin
    }
}

impl DerefMut for ComponentBin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bin
    }
}

#[derive(Debug, Clone)]
pub struct Bridge {
    components: Vec<Component>,
    end: usize,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            components: Vec::new(),
            end: 0,
        }
    }

    fn add(&mut self, component: Component) {
        self.components.push(component);
        self.end = component.other(self.end).expect("component doesn't fit");
    }

    fn remove(&mut self) -> Option<Component> {
        self.components.pop().map(|removed| {
            self.end = removed.other(self.end).expect("how'd that get there?");
            removed
        })
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn strength(&self) -> usize {
        self.components
            .iter()
            .map(|c| c.left + c.right)
            .sum::<usize>()
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_strs = self.components
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("-");
        write!(f, "{}: {}", c_strs, self.end)
    }
}

pub struct BridgeBuilder {
    parts: ComponentBin,
    bridge: Bridge,
    nstack: Vec<usize>,
}

impl BridgeBuilder {
    pub fn new(components: &[Component]) -> BridgeBuilder {
        let mut bin = ComponentBin::new();
        for component in components {
            bin.insert(*component);
        }
        BridgeBuilder {
            parts: bin,
            bridge: Bridge::new(),
            nstack: vec![0],
        }
    }
}

impl Iterator for BridgeBuilder {
    type Item = Bridge;
    fn next(&mut self) -> Option<Bridge> {
        if self.nstack.len() == 0 {
            return None;
        }
        // if we can add a component to the existing bridge, do that
        // take out the nth biggest component matching the current end
        if let Some(next_component) = self.parts
            .pop_nth(self.bridge.end, self.nstack[self.nstack.len() - 1])
        {
            // next call needs to get the subsequent item from the parts bin
            {
                let last_idx = self.nstack.len() - 1;
                self.nstack[last_idx] += 1;
            }

            // actually, next call is one level deeper
            self.nstack.push(0);
            self.bridge.add(next_component);

            Some(self.bridge.clone())
        } else {
            // back down the bridge and n stacks until we
            // get to another valid
            let mut success = false;
            while self.nstack.len() > 1 {
                self.nstack.pop();
                let component = self.bridge.remove().expect("bridge must be long enough");
                self.parts.insert(component);

                if let Some(sl) = self.parts.size_len(self.bridge.end) {
                    if self.nstack.len() > 0 && self.nstack[self.nstack.len() - 1] < sl {
                        success = true;
                        break;
                    }
                }
            }
            if success {
                self.next()
            } else {
                None
            }
        }
    }
}

pub fn max_strength_bridge(components: &[Component]) -> Option<Bridge> {
    BridgeBuilder::new(components).max_by_key(|bridge| bridge.strength())
}

pub fn max_length_strength_bridge(components: &[Component]) -> Option<Bridge> {
    BridgeBuilder::new(components).max_by(|b1, b2| match b1.len().cmp(&b2.len()) {
        Ordering::Equal => b1.strength().cmp(&b2.strength()),
        otherwise => otherwise,
    })
}
