use crate::{Dfa};
use std::fmt::Write;
use std::collections::HashMap;

impl Dfa {
    pub fn print_matrix(&self) -> String {
        let mut s = String::new();
        for (idx, node) in self.nodes.iter().enumerate() {
            let mut outs = HashMap::new();
            for (&k, &out) in &node.1 {
                outs.entry(out).or_insert_with(Vec::new).push(k);
            }

            for (out, mut edge) in outs {
                edge.sort_unstable();

                let trans_char = String::from_utf8(edge).unwrap();

                if let Err(e) = writeln!(s, r#"{} {} {}"#, idx, out, trans_char) {
                    println!("Writing error: {}", e.to_string());
                }
            }
        }
        
        s
    }
}
