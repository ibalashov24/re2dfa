use crate::Dfa;
use std::collections::HashMap;
use std::fmt::Write;
use std::char;

impl Dfa {
    /// Prints DFA as adjacency matrix *state x char*.
    /// # Returns
    /// String containing adjacency matrix.
    pub fn print_dense_matrix(&self) -> String {
        let mut result_str = String::new();

        for node in &self.nodes {
            let mut matrix_row = HashMap::new();
            for (&trans_symbol, &out) in &node.1 {
                matrix_row.insert(trans_symbol, out);
            }

            // 255 is byte's (and ASCII's) upper border
            for symbol in 0u8..=255 {
                if let Some(out) = matrix_row.get(&symbol) {
                    let _ = write!(result_str, r#"{} "#, out); 
                } else {
                    let _ = write!(result_str, "0 ");
                }
            }
            let _ = writeln!(result_str);
        }

       result_str 
    }

    /// Prints DFA as COOrdinate list *(from, symbol, to)*.
    /// # Returns
    /// String containing COOrdiante list.
    pub fn print_coo(&self) -> String {
        let mut result_str = String::new(); 
            
        for (idx, node) in self.nodes.iter().enumerate() {
           for (&trans_symbol, &out) in &node.1 {
               let _ = writeln!(
                   result_str, 
                   r#"{} {} {}"#, 
                    idx, 
                    char::from_u32(trans_symbol as u32).unwrap(), 
                    out);  
           }
        }

        result_str
    }
}
