use crate::hir::path::*;
use speculate::speculate;

speculate!{
    before {
        #[allow(unused_variables)]
        let generate_path_tree = |nodes: Vec<HirPathNode>| {
            let mut path_tree = HirPathTree::new();
            let mut index_generator = HirPathIndexGenerator::new();

            for each_node in nodes {
                path_tree.add_node(&mut index_generator, None, each_node);
            }

            path_tree
        };
    }

    describe "index generator" {
        it "returns index that is not increased as None" {
            let index_generator = HirPathIndexGenerator::new();
            assert_eq!(index_generator.index(), None);
        }

        it "increases index" {
            let mut index_generator = HirPathIndexGenerator::new();
            assert_eq!(index_generator.generate(), 0.into());
            assert_eq!(index_generator.index(), Some(0.into()));
            assert_eq!(index_generator.generate(), 1.into());
            assert_eq!(index_generator.index(), Some(1.into()));
        }
    }

    describe "node addition" {
        #[should_panic(expected = "hako path node can't have a parent")]
        it "panics when adds hako node that has parent" {
            let node = HirPathNode {
                id: "hako_node_with_parent".into(),
                kind: HirPathKind::Hako,
                parent: Some(0.into()),
                children: Vec::new(),
            };

            generate_path_tree(vec![node.clone()]);
        }
    }

    describe "node acquirement" {
        it "gets node by index" {
            let node = HirPathNode {
                id: "node".into(),
                kind: HirPathKind::Hako,
                parent: None,
                children: Vec::new(),
            };

            let path_tree = generate_path_tree(vec![node.clone()]);
            assert_eq!(path_tree.get(&0.into()), Some(&node));
        }

        it "finds node by segments" {
            let hako_node = HirPathNode {
                id: "node".into(),
                kind: HirPathKind::Hako,
                parent: None,
                children: vec![1.into()],
            };

            let subnode = HirPathNode {
                id: "subnode".into(),
                kind: HirPathKind::Module,
                parent: Some(0.into()),
                children: Vec::new(),
            };

            let path_tree = generate_path_tree(vec![hako_node, subnode.clone()]);
            assert_eq!(path_tree.find(&vec!["node".into(), "subnode".into()]), Some((&1.into(), &subnode)));
        }

        it "finds hako node" {
            let node = HirPathNode {
                id: "node".into(),
                kind: HirPathKind::Hako,
                parent: None,
                children: Vec::new(),
            };

            let path_tree = generate_path_tree(vec![node.clone()]);
            assert_eq!(path_tree.find_hako(&"node".into()), Some((&0.into(), &node)));
        }

        it "finds child node" {
            let node = HirPathNode {
                id: "node".into(),
                kind: HirPathKind::Hako,
                parent: None,
                children: Vec::new(),
            };

            let path_tree = generate_path_tree(vec![node.clone()]);
            assert_eq!(path_tree.find_child(&vec![0.into()], &"node".into()), Some((&0.into(), &node)));
        }
    }
}
