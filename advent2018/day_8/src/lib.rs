pub struct Node {
    pub metadata: Vec<usize>,
    pub childs: Vec<Node>,
    pub sum: Option<usize>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            metadata: Vec::new(),
            childs: Vec::new(),
            sum: None,
        }
    }
    pub fn raw_metadata_sum(&self) -> usize {
        let self_sum: usize = self.metadata.iter().sum();
        let child_sum: usize = self.childs.iter().map(|node| node.raw_metadata_sum()).sum();
        return self_sum + child_sum;
    }
    pub fn metadata_sum(&mut self) -> usize {
        if self.sum != None {
            return self.sum.unwrap();
        }
        let number_of_childs = self.childs.len();
        if number_of_childs == 0 {
            self.sum = Some(self.metadata.iter().sum::<usize>());
            return self.sum.unwrap();
        }
        let cloned_metadata = self.metadata.clone();

        self.sum = Some(
            cloned_metadata
                .iter()
                .filter(|child_number| **child_number <= number_of_childs)
                .map(|child_number| self.childs[*child_number - 1].metadata_sum())
                .sum::<usize>(),
        );
        self.sum.unwrap()
    }
}

pub fn parse_node(mut license: &[usize]) -> (Node, &[usize]) {
    let nbr_of_child = license[0];
    let nbr_of_meta = license[1];

    let mut node = Node::new();
    license = &license[2..];
    for _ in 0..nbr_of_child {
        let (child, new_license) = parse_node(license);
        license = new_license;
        node.childs.push(child);
    }
    for index in 0..nbr_of_meta {
        node.metadata.push(license[index]);
    }
    return (node, &license[(nbr_of_meta)..]);
}
