// Translation from Sluice to p4 for each snippet.
// This works by first constructing a DAG.
use grammar::*;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DagNodeType<'a> {
    Decl(&'a VariableDecl<'a>),
    Cond(&'a Expr<'a>),
    Stmt(&'a Statement<'a>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DagNode<'a> {
    pub node_type : DagNodeType<'a>,
    pub next_nodes : Vec<usize>,
    pub prev_nodes : Vec<usize>
}

// For now, using a simplistic DAG dc using vectors.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Dag<'a> {
    pub dag_vector : Vec<DagNode<'a>>
}

pub fn getIdentifiers<'a> (my_operand : &'a Operand<'a>) -> Vec<&'a str> {
    match &my_operand {
        Operand::LValue(ref lval) => {
            let mut nex_vec = lval.get_string_vec();
            return nex_vec;
        },
        _ =>  { return Vec::new(); }
    }

}

pub fn get_indices_lval<'a> (decl_map : &HashMap<&str, usize>,
    lval : &'a LValue<'a>) -> HashMap<&'a str, usize> {
    let mut my_indices : HashMap<&'a str, usize> = HashMap::new();

    let my_vec_ids = &lval.get_string_vec();
    for my_id in my_vec_ids {
        println!("{:?} ", my_id);
        let my_option = decl_map.get(my_id);
        match my_option {
            Some(index) => {
                my_indices.insert(my_id, *index);
            }
            None => {}
        }
    }
    return my_indices;

}

pub fn get_indices_op<'a> (decl_map : &HashMap<&str, usize>, op : &'a Operand<'a>) -> HashMap<&'a str, usize> {
    let mut empty : HashMap<&str, usize> = HashMap::new();
    match &op {
        Operand::LValue(ref lval) => {
            return get_indices_lval(decl_map, lval);
        },
        _ =>  { return empty; }
    }
}

pub fn get_dag_node<'a>(my_dag : &'a Dag<'a>,index : &usize) ->  Option<&'a DagNode<'a>> {
    let my_dag_option = &my_dag.dag_vector.get(*index);
    return *my_dag_option;

}
//pub fn build_map<'a> (my_snippet: &'a Snippet<'a>) ->
// Construct the connections between the nodes to form the Dag
// TODO : Make it modular. Curently baffled by how to pass mutable reference of Dag again
pub fn create_connections<'a> (my_snippet: &'a Snippet<'a>, my_dag : &mut Dag<'a>) {
    // A HashMap to keep track of declarations.
    let mut decl_map : HashMap<&str, usize>= HashMap::new();
    //First, process variable decls
    let mut i : usize = 0;
    for my_variable_decl in &my_snippet.variable_decls.decl_vector {
        decl_map.insert(&my_variable_decl.identifier.id_name, i);
        i = i + 1;
    }
    println!("decl map: {:?}\n ", decl_map);
    // Next, process statements, for now ignoring if block.
    for my_if_block in &my_snippet.ifblocks.ifblock_vector {
        for my_statement in &my_if_block.statements.stmt_vector {
            println!("Statement : {:?}: ",  my_statement );
            let mut my_indices_1 : HashMap<&str, usize> = HashMap::new();
            let mut my_indices_2 : HashMap<&str, usize> = HashMap::new();
            let mut my_indices_3 : HashMap<&str, usize> = HashMap::new();
            let mut my_indices_4 : HashMap<&str, usize> = HashMap::new();
            let mut my_indices_5 : HashMap<&str, usize> = HashMap::new();
            // Connect based on LValue of statements
            my_indices_1 = get_indices_lval(&decl_map, &my_statement.lvalue);
            // Connect based on the first operand
            my_indices_2 = get_indices_op(&decl_map, &my_statement.expr.op1);
            // Connect based on the rest of operand
            match &my_statement.expr.expr_right {
                ExprRight::BinOp(_btype, op2) => {
                    my_indices_3 = get_indices_op(&decl_map, &op2);
                }
                ExprRight::Cond(op_true, op_false) => {
                    my_indices_4 = get_indices_op(&decl_map, &op_true);
                    my_indices_5 = get_indices_op(&decl_map, &op_false);
                }
                ExprRight::Empty() => {

                }
            }
            // Populate next_nodes
            for (my_id_1,p_index_1) in my_indices_1.clone() {
                let my_parent_dag_option = my_dag.dag_vector.get_mut(p_index_1);
                match my_parent_dag_option {
                    Some(mut my_parent_dag_node) => {
                        if !&my_parent_dag_node.next_nodes.contains(&i) {
                            my_parent_dag_node.next_nodes.push(i);
                            println!("parent_dag_node {:?}", my_parent_dag_node);

                        }
                    }
                    None => {}
                }
                decl_map.insert(my_id_1, i);
            }
            for (my_id_2,p_index_2) in my_indices_2.clone() {
                let my_parent_dag_option = my_dag.dag_vector.get_mut(p_index_2);
                match my_parent_dag_option {
                    Some(mut my_parent_dag_node) => {
                        if !&my_parent_dag_node.next_nodes.contains(&i) {
                            my_parent_dag_node.next_nodes.push(i);
                            println!("parent_dag_node {:?}", my_parent_dag_node);
                        }
                    }
                    None => {}
                }
                decl_map.insert(my_id_2, i);
            }
            for (my_id_3,p_index_3) in my_indices_3.clone() {
                let my_parent_dag_option = my_dag.dag_vector.get_mut(p_index_3);
                match my_parent_dag_option {
                    Some(mut my_parent_dag_node) => {
                        if !&my_parent_dag_node.next_nodes.contains(&i) {
                            my_parent_dag_node.next_nodes.push(i);
                            //println!("parent_dag_node {:?}", my_parent_dag_node);
                        }
                    }
                    None => {}
                }
                decl_map.insert(my_id_3, i);
            }
            for (my_id_4,p_index_4) in my_indices_4.clone() {
                let my_parent_dag_option = my_dag.dag_vector.get_mut(p_index_4);
                match my_parent_dag_option {
                    Some(mut my_parent_dag_node) => {
                        if !&my_parent_dag_node.next_nodes.contains(&i) {
                            my_parent_dag_node.next_nodes.push(i);
                            //println!("parent_dag_node {:?}", my_parent_dag_node);
                        }
                    }
                    None => {}
                }
                decl_map.insert(my_id_4, i);
            }
            for (my_id_5,p_index_5) in my_indices_5.clone() {
                let my_parent_dag_option = my_dag.dag_vector.get_mut(p_index_5);
                match my_parent_dag_option {
                    Some(mut my_parent_dag_node) => {
                        if !&my_parent_dag_node.next_nodes.contains(&i) {
                            my_parent_dag_node.next_nodes.push(i);
                            //println!("parent_dag_node {:?}", my_parent_dag_node);
                        }
                    }
                    None => {}
                }
                decl_map.insert(my_id_5, i);
            }
            // Populate prev_nodes
            for (_my_id_1,p_index_1) in my_indices_1 {
                let my_dag_option = my_dag.dag_vector.get_mut(i);
                match my_dag_option {
                    Some(mut my_dag_node) => {
                        if !&my_dag_node.prev_nodes.contains(&p_index_1) {
                            my_dag_node.prev_nodes.push(p_index_1);
                            println!("prev parent_dag_node {:?}", my_dag_node);

                        }
                    }
                    None => {}
                }
            }
            for (_my_id_2,p_index_2) in my_indices_2 {
                let my_dag_option = my_dag.dag_vector.get_mut(i);
                match my_dag_option {
                    Some(mut my_dag_node) => {
                        if !&my_dag_node.prev_nodes.contains(&p_index_2) {
                            my_dag_node.prev_nodes.push(p_index_2);
                            println!("prev parent_dag_node {:?}", my_dag_node);
                        }
                    }
                    None => {}
                }
            }
            for (_my_id_3,p_index_3) in my_indices_3 {
                let my_dag_option = my_dag.dag_vector.get_mut(i);
                match my_dag_option {
                    Some(mut my_dag_node) => {
                        if !&my_dag_node.prev_nodes.contains(&p_index_3) {
                            my_dag_node.prev_nodes.push(p_index_3);
                        }
                    }
                    None => {}
                }
            }
            for (_my_id_4,p_index_4) in my_indices_4 {
                let my_dag_option = my_dag.dag_vector.get_mut(i);
                match my_dag_option {
                    Some(mut my_dag_node) => {
                        if !&my_dag_node.prev_nodes.contains(&p_index_4) {
                            my_dag_node.prev_nodes.push(p_index_4);
                        }
                    }
                    None => {}
                }
            }
            for (_my_id_5,p_index_5) in my_indices_5 {
                let my_dag_option = my_dag.dag_vector.get_mut(i);
                match my_dag_option {
                    Some(mut my_dag_node) => {
                        if !&my_dag_node.prev_nodes.contains(&p_index_5) {
                            my_dag_node.prev_nodes.push(p_index_5);
                        }
                    }
                    None => {}
                }
            }
            println!("decl map: {:?}\n ", decl_map);
            i = i+1;
        }
    }
}

pub fn create_dag_nodes<'a> (my_snippets : &'a Snippets) -> HashMap<&'a str, Dag<'a>>  {
    let mut dag_map : HashMap<&str, Dag>= HashMap::new();

    for my_snippet in &my_snippets.snippet_vector {
        let mut my_dag : Dag = Dag { dag_vector : Vec::new()};
        for my_snippet in &my_snippets.snippet_vector {
            //let my_dag_start_node : DagNode;

            for my_variable_decl in &my_snippet.variable_decls.decl_vector {
                let my_dag_start_node = DagNode {node_type : DagNodeType::Decl(my_variable_decl),
                      next_nodes : Vec::new(), prev_nodes : Vec::new()};
                my_dag.dag_vector.push(my_dag_start_node);
            }
            for my_if_block in &my_snippet.ifblocks.ifblock_vector {
                for my_statement in &my_if_block.statements.stmt_vector {
                    let mut my_dag_node = DagNode {node_type: DagNodeType::Stmt(&my_statement),
                         next_nodes: Vec::new(), prev_nodes: Vec::new()};
                    my_dag.dag_vector.push(my_dag_node);
                }
            }
        }
        dag_map.insert(&my_snippet.snippet_id.id_name, my_dag);
    }
    dag_map
}


pub fn trans_snippets<'a> (my_snippets : &Snippets<'a>) {
    // TODO : Deal with mutability of my_dag
    let mut dag_map = create_dag_nodes(&my_snippets);
    //println!("Dag Map: {:?}\n", dag_map);
    for my_snippet in &my_snippets.snippet_vector {
        let mut my_option = dag_map.get_mut(&my_snippet.snippet_id.id_name);
        match my_option {
            Some(mut snippet_dag) => {
                create_connections(&my_snippet, &mut snippet_dag);
                println!("Snippet DAG: {:?}\n", snippet_dag);
            }
            None => {}

        }
    }
}