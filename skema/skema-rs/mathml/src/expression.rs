use crate::ast::{
    Math, MathExpression,
    MathExpression::{Mfrac, Mi, Mn, Mo, Mover, Mrow, Msqrt, Msubsup, Msup},
    Operator,
};
use std::clone::Clone;

use petgraph::{graph::NodeIndex, Graph};

use crate::petri_net::recognizers::is_leibniz_diff_operator;
use std::collections::VecDeque;

/// Struct for representing mathematical expressions in order to align with source code.
pub type MathExpressionGraph<'a> = Graph<String, String>;

use std::string::ToString;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(String),
    Identifier(String),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub ops: Vec<Operator>,
    pub args: Vec<Expr>,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(Atom),
    Expression {
        ops: Vec<Operator>,
        args: Vec<Expr>,
        name: String,
    },
}

/// Intermediate data structure to support the generation of graphs of mathematical expressions
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PreExp {
    pub ops: Vec<Operator>,
    pub args: Vec<Expr>,
    pub name: String,
}

/// Check if the fraction is a derivative expressed in Leibniz notation. If yes, mutate it to
/// remove the 'd' prefixes.
pub fn is_derivative(
    numerator: &mut Box<MathExpression>,
    denominator: &mut Box<MathExpression>,
) -> bool {
    if is_leibniz_diff_operator(numerator, denominator) {
        if let Mrow(x) = &mut **numerator {
            x.remove(0);
        }

        if let Mrow(x) = &mut **denominator {
            x.remove(0);
        }
        return true;
    }
    false
}

/// Identify if there is an implicit multiplication operator, and if so, add an
/// explicit multiplication operator.
fn insert_explicit_multiplication_operator(pre: &mut PreExp) {
    if pre.args.len() >= pre.ops.len() {
        pre.ops.push(Operator::Multiply);
    }
}

impl MathExpression {
    /// Convert a MathExpression struct to a PreExp struct.
    pub fn to_expr(self, pre: &mut PreExp) {
        match self {
            Mi(x) => {
                // Process unary minus operation.
                if !pre.args.is_empty() {
                    // Check the last arg
                    let args_last_idx = pre.args.len() - 1;
                    if let Expr::Atom(Atom::Operator(Operator::Subtract)) = &pre.args[args_last_idx]
                    {
                        let neg_identifier = format!("-{x}");
                        pre.args[args_last_idx] = Expr::Atom(Atom::Identifier(neg_identifier));
                        return;
                    }
                }
                // deal with the invisible multiply operator
                if pre.args.len() >= pre.ops.len() {
                    pre.ops.push(Operator::Multiply);
                }
                pre.args
                    .push(Expr::Atom(Atom::Identifier(x.replace(' ', ""))));
            }
            Mn(x) => {
                insert_explicit_multiplication_operator(pre);
                // Remove redundant whitespace
                pre.args.push(Expr::Atom(Atom::Number(x.replace(' ', ""))));
            }
            Mo(x) => {
                // Insert a temporary placeholder identifier to deal with unary minus operation.
                // The placeholder will be removed later.
                if x == Operator::Subtract && pre.ops.len() > pre.args.len() {
                    pre.ops.push(x);
                    pre.args
                        .push(Expr::Atom(Atom::Identifier("place_holder".to_string())));
                } else {
                    pre.ops.push(x);
                }
            }
            Mrow(xs) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                pre_exp.ops.push(Operator::Other("".to_string()));
                for x in xs {
                    x.to_expr(&mut pre_exp);
                }
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            Msubsup(xs1, xs2, xs3) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                pre_exp.ops.push(Operator::Other("".to_string()));
                pre_exp.ops.push(Operator::Other("_".to_string()));
                xs1.to_expr(&mut pre_exp);
                pre_exp.ops.push(Operator::Other("^".to_string()));
                xs2.to_expr(&mut pre_exp);
                xs3.to_expr(&mut pre_exp);
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            Msqrt(xs) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                pre_exp.ops.push(Operator::Sqrt);
                xs.to_expr(&mut pre_exp);
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            Mfrac(mut xs1, mut xs2) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                if is_derivative(&mut xs1, &mut xs2) {
                    pre_exp.ops.push(Operator::Other("derivative".to_string()));
                } else {
                    pre_exp.ops.push(Operator::Other("".to_string()));
                }
                xs1.to_expr(&mut pre_exp);
                pre_exp.ops.push(Operator::Divide);
                xs2.to_expr(&mut pre_exp);
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            Msup(xs1, xs2) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                pre_exp.ops.push(Operator::Other("".to_string()));
                xs1.to_expr(&mut pre_exp);
                pre_exp.ops.push(Operator::Other("^".to_string()));
                xs2.to_expr(&mut pre_exp);
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            Mover(xs1, xs2) => {
                insert_explicit_multiplication_operator(pre);
                let mut pre_exp = PreExp::default();
                pre_exp.ops.push(Operator::Other("".to_string()));
                xs1.to_expr(&mut pre_exp);
                xs2.to_expr(&mut pre_exp);
                pre_exp.ops.remove(0);
                pre.args.push(Expr::Expression {
                    ops: pre_exp.ops,
                    args: pre_exp.args,
                    name: "".to_string(),
                });
            }
            _ => {
                panic!("Unhandled type!");
            }
        }
    }

    pub fn to_graph(self) -> MathExpressionGraph<'static> {
        let mut pre_exp = PreExp {
            ops: Vec::<Operator>::new(),
            args: Vec::<Expr>::new(),
            name: "root".to_string(),
        };
        pre_exp.ops.push(Operator::Other("root".to_string()));
        self.to_expr(&mut pre_exp);
        pre_exp.group_expr();
        pre_exp.collapse_expr();
        /// if need to convert to canonical form, please uncomment the following
        // pre_exp.distribute_expr();
        // pre_exp.group_expr();
        // pre_exp.collapse_expr();
        pre_exp.set_name();

        pre_exp.to_graph()
    }
}

impl Expr {
    /// Group expression by multiplication and division operations.
    pub fn group_expr(&mut self) {
        if let Expr::Expression { ops, args, .. } = self {
            let mut indices_to_remove = Vec::new();
            let ops_copy = ops.clone();
            let args_copy = args.clone();
            if ops.len() > 2 {
                let mut start_index: i32 = -1;
                let mut end_index: i32 = -1;
                let mut new_exp = Expr::Expression {
                    ops: vec![Operator::Other("".to_string())],
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                for (o, operator) in ops.iter().enumerate() {
                    if *operator == Operator::Multiply || *operator == Operator::Divide {
                        indices_to_remove.push(o);
                        if start_index == -1 {
                            start_index = o as i32;
                            end_index = o as i32;
                            if let Expr::Expression { ops, args, .. } = &mut new_exp {
                                ops.push(operator.clone());
                                args.push(args_copy[o - 1].clone());
                                args.push(args_copy[o].clone());
                            }
                        } else if o as i32 - end_index == 1 {
                            end_index = o as i32;
                            if let Expr::Expression { ops, args, .. } = &mut new_exp {
                                ops.push(ops_copy[o].clone());
                                args.push(args_copy[o].clone())
                            }
                        } else {
                            args[start_index as usize - 1] = new_exp.clone();
                            new_exp = Expr::Expression {
                                ops: vec![Operator::Other("".to_string())],
                                args: Vec::<Expr>::new(),
                                name: "".to_string(),
                            };
                            if let Expr::Expression { ops, args, .. } = &mut new_exp {
                                ops.push(ops_copy[o].clone());
                                args.push(args_copy[o - 1].clone());
                                args.push(args_copy[o].clone());
                            }
                            start_index = o as i32;
                            end_index = o as i32;
                        }
                    }
                }

                if indices_to_remove.len() == ops.len() - 1 {
                    return;
                }

                if let Expr::Expression { ops, .. } = &mut new_exp {
                    if !ops.is_empty() && start_index > 0 {
                        args[start_index as usize - 1] = new_exp.clone();
                    }
                }
                for ri in indices_to_remove.iter().rev() {
                    ops.remove(*ri);
                    args.remove(*ri);
                }
            }

            for arg in args {
                if let Expr::Expression { .. } = arg {
                    arg.group_expr();
                }
            }
        }
    }

    /// 1) If the current term's operators are all multiplication or division, check if it contains
    /// nested all multiplication or division terms inside. If so, collapse them into a single term.
    /// 2) If the current term's operators are all summation or subtraction, check if it contains
    /// nested all summation or subtraction terms inside. If so, collapse them into a single term.
    fn collapse_expr(&mut self) {
        if let Expr::Expression { ops, args, .. } = self {
            let mut ops_copy = ops.clone();
            let mut args_copy = args.clone();
            let mut shift = 0;
            if all_ops_are_mult_or_div(ops.to_vec()) && ops.len() > 1 {
                let mut changed = true;
                while changed {
                    shift = 0;
                    for i in 0..args.len() {
                        let mut is_div: bool = false;
                        if ops[i] == Operator::Divide {
                            is_div = true;
                        };
                        if let Expr::Expression { ops, args, name: _ } = &mut args[i] {
                            if ops[0] == Operator::Other("".to_string())
                                && all_ops_are_mult_or_div(ops.to_vec())
                            {
                                args_copy[i + shift] = args[0].clone();
                                for j in 1..ops.len() {
                                    if is_div {
                                        ops_copy.insert(
                                            i + shift + j,
                                            switch_mul_div(ops[j].clone()).clone(),
                                        );
                                    } else {
                                        ops_copy.insert(i + shift + j, ops[j].clone());
                                    }
                                    args_copy.insert(i + shift + j, args[j].clone());
                                }
                                shift = shift + ops.len() - 1;
                            }
                        }
                    }
                    if ops.clone() == ops_copy.clone() {
                        changed = false;
                    }
                    *ops = ops_copy.clone();
                    *args = args_copy.clone();
                }
            }

            if all_ops_are_add_or_subt(ops.to_vec()) && ops.len() > 1 {
                let mut changed = true;
                while changed {
                    shift = 0;
                    for i in 0..args.len() {
                        let mut is_subt: bool = false;
                        if ops[i] == Operator::Subtract {
                            is_subt = true;
                        };
                        if let Expr::Expression { ops, args, name: _ } = &mut args[i] {
                            if ops[0] == Operator::Other("".to_string())
                                && all_ops_are_add_or_subt(ops.to_vec())
                            {
                                args_copy[i + shift] = args[0].clone();
                                for j in 1..ops.len() {
                                    if is_subt {
                                        ops_copy.insert(
                                            i + shift + j,
                                            switch_add_subt(ops[j].clone()).clone(),
                                        );
                                    } else {
                                        ops_copy.insert(i + shift + j, ops[j].clone());
                                    }
                                    args_copy.insert(i + shift + j, args[j].clone());
                                }
                                shift = shift + ops.len() - 1;
                            }
                        }
                    }

                    if ops.clone() == ops_copy.clone() {
                        changed = false;
                    }
                    *ops = ops_copy.clone();
                    *args = args_copy.clone();
                }
            }

            for arg in args {
                arg.collapse_expr();
            }
        }
    }

    /// 1) distribute variables and terms over multiplications, e.g., a*(b+c) => a*b+a*c
    /// 2) distribute variables and terms over divisions, e.g., a/(b+c)/(e+f) => a/(be+bf+ce+cf)
    fn distribute_expr(&mut self) {
        if let Expr::Expression { ops, args, .. } = self {
            let mut ops_copy = ops.clone();
            let mut args_copy = args.clone();
            let mut distributed_ops: Vec<Operator> = Default::default();
            let mut distributed_terms: Vec<Expr> = Default::default();
            let mut is_distributed: bool = false;
            if ops_contain_mult(ops.to_vec()) && ops.len() > 1 {
                for i in 0..args.len() {
                    if i == 0 || ops[i] == Operator::Multiply {
                        if let Expr::Expression { ops, args, name: _ } = &mut args[i] {
                            if need_to_distribute(ops.to_vec()) {
                                is_distributed = true;
                                for j in 0..args.len() {
                                    let mut distributed_ops_unit = ops_copy.clone();
                                    let mut distributed_terms_unit = args_copy.clone();
                                    if j == 0 {
                                        distributed_ops.extend(distributed_ops_unit);
                                    } else {
                                        distributed_ops_unit[0] = ops[j].clone();
                                        distributed_ops.extend(distributed_ops_unit);
                                    }
                                    distributed_terms_unit[i] = args[j].clone();
                                    distributed_terms.extend(distributed_terms_unit);
                                }
                            }
                        }
                    }
                    if is_distributed {
                        let mut new_expr = Expr::Expression {
                            ops: distributed_ops.clone(),
                            args: distributed_terms.clone(),
                            name: "".to_string(),
                        };
                        new_expr.group_expr();
                        new_expr.collapse_expr();
                        new_expr.distribute_expr();
                        new_expr.group_expr();
                        new_expr.collapse_expr();
                        if let Expr::Expression { ops, args, name: _ } = &mut new_expr {
                            ops_copy = ops.clone();
                            args_copy = args.clone();
                        }
                        *ops = ops_copy.clone();
                        *args = args_copy.clone();
                        break;
                    }
                }
            }

            if need_to_distribute_divs(ops.to_vec().clone(), args.to_vec().clone()) && ops.len() > 1
            {
                let mut new_expr = Expr::Expression {
                    ops: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                if let Expr::Expression { ops, args, name: _ } = &mut new_expr {
                    ops.push(Operator::Other("".to_string()));
                }
                let mut removed_idx: Vec<usize> = Vec::new();
                for i in 0..args.len() {
                    if ops[i] == Operator::Divide {
                        removed_idx.push(i);
                        let mut tmp_arg = args[i].clone();
                        if let Expr::Expression { ops, args, name: _ } = &mut new_expr {
                            if ops.len() == args.len() {
                                ops.push(Operator::Multiply);
                            }
                            args.push(tmp_arg.clone());
                        }
                    }
                }
                new_expr.group_expr();
                new_expr.collapse_expr();
                new_expr.distribute_expr();
                new_expr.group_expr();
                new_expr.collapse_expr();

                for ri in removed_idx.iter().rev() {
                    ops.remove(*ri);
                    args.remove(*ri);
                }

                ops.push(Operator::Divide);
                args.push(new_expr);
            }

            for arg in args {
                arg.distribute_expr();
            }
        }
    }

    /// Construct a string representation of the Expression and store it under its 'name' property.
    pub fn set_name(&mut self) -> String {
        let mut add_paren = false;
        match self {
            Expr::Atom(_) => "".to_string(),
            Expr::Expression { ops, args, name } => {
                if ops[0] == Operator::Other("".to_string())
                    && !all_ops_are_mult_or_div(ops.to_vec())
                    && !contains_redundant_parens(name)
                {
                    name.push('(');
                    add_paren = true;
                }
                for i in 0..=ops.len() - 1 {
                    if i > 0 {
                        if ops[i] == Operator::Equals {
                            let mut new_name: String = "".to_string();
                            for n in name.as_bytes().clone() {
                                if *n == 40_u8 {
                                    new_name.push('(');
                                }
                                if *n == 41_u8 {
                                    new_name.push(')');
                                }
                            }
                            *name = remove_redundant_parens(&mut new_name).clone();
                        } else {
                            name.push_str(&ops[i].to_string().clone());
                        }
                    }
                    match &mut args[i] {
                        Expr::Atom(x) => match x {
                            Atom::Number(x) => {
                                name.push_str(&x.to_string().clone());
                            }
                            Atom::Identifier(x) => {
                                name.push_str(&x.to_string().clone());
                            }
                            Atom::Operator(_) => {}
                        },
                        Expr::Expression { ops, .. } => {
                            let mut string;
                            if ops[0] != Operator::Other("".to_string()) {
                                string = ops[0].to_string();
                                string.push('(');
                                string.push_str(args[i].set_name().as_str().clone());
                                string.push(')');
                            } else {
                                string = args[i].set_name().as_str().to_string().clone();
                            }
                            name.push_str(&string.clone());
                        }
                    }
                }
                if add_paren {
                    name.push(')');
                }
                add_paren = false;

                name.to_string()
            }
        }
    }

    pub fn to_graph(&mut self, graph: &mut MathExpressionGraph) {
        if let Expr::Expression { ops, args, name } = self {
            if name == "place_holder" {
                return;
            } else if name.contains("place_holder") {
                *name = name.replace("place_holder", "");
            }

            let mut parent_node_index: NodeIndex = Default::default();
            if ops[0].to_string() != "derivative" {
                parent_node_index = get_node_idx(graph, name)
            }
            let mut eq_loc = 0;
            if ops.contains(&Operator::Equals) {
                eq_loc = ops.iter().position(|r| r == &(Operator::Equals)).unwrap();
                let mut left_eq_name: String = "".to_string();
                for i in 0..eq_loc {
                    match &mut args[i] {
                        Expr::Atom(x) => match x {
                            Atom::Number(y) => {
                                left_eq_name.push_str(y);
                            }
                            Atom::Identifier(y) => {
                                left_eq_name.push_str(y);
                            }
                            Atom::Operator(_y) => {}
                        },
                        Expr::Expression { ops, args: _, name } => {
                            if ops[0] != Operator::Other("".to_string()) {
                                let mut unitary_name = ops[0].to_string();
                                let mut name_copy = name.to_string();
                                remove_redundant_parens(&mut name_copy);
                                unitary_name.push_str("(".clone());
                                unitary_name.push_str(&name_copy.clone());
                                unitary_name.push_str(")".clone());
                                left_eq_name.push_str(unitary_name.as_str());
                            } else {
                                left_eq_name.push_str(name.as_str());
                            }
                        }
                    }
                }

                let node_idx = get_node_idx(graph, &mut left_eq_name);
                graph.update_edge(node_idx, parent_node_index, "=".to_string());
            }
            if ops[0] != Operator::Other("".to_string()) {
                let mut unitary_name = ops[0].to_string();
                let mut name_copy = name.to_string();
                remove_redundant_parens(&mut name_copy);
                unitary_name.push_str("(".clone());
                unitary_name.push_str(&name_copy.clone());
                unitary_name.push_str(")".clone());
                let node_idx = get_node_idx(graph, &mut unitary_name);
                if ops[0].to_string() == "derivative" {
                    return;
                } else {
                    graph.update_edge(parent_node_index, node_idx, ops[0].to_string());
                }
            }
            let ops_copy = ops.clone();
            for i in eq_loc..=ops_copy.len() - 1 {
                match &mut args[i] {
                    Expr::Atom(x) => match x {
                        Atom::Number(x) => {
                            if x == "place_holder" {
                                continue;
                            } else if x.contains("place_holder") {
                                *x = x.replace("place_holder", "");
                            }
                            let node_idx = get_node_idx(graph, x);
                            if i == 0 {
                                if ops_copy.len() > 1 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !x.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else if ops_copy[i] == Operator::Equals {
                                if i <= ops_copy.len() - 2 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !x.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else {
                                graph.update_edge(
                                    node_idx,
                                    parent_node_index,
                                    ops_copy[i].to_string(),
                                );
                            }
                        }
                        Atom::Identifier(x) => {
                            if x == "place_holder" {
                                continue;
                            } else if x.contains("place_holder") {
                                *x = x.replace("place_holder", "");
                            }
                            let node_idx = get_node_idx(graph, x);
                            if i == 0 {
                                if ops_copy.len() > 1 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !x.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else if ops_copy[i] == Operator::Equals {
                                if i <= ops_copy.len() - 2 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !x.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else {
                                graph.update_edge(
                                    node_idx,
                                    parent_node_index,
                                    ops_copy[i].to_string(),
                                );
                            }
                        }
                        Atom::Operator(_x) => {}
                    },
                    Expr::Expression { ops, name, .. } => {
                        if name == "place_holder" {
                            continue;
                        } else if name.contains("place_holder") {
                            *name = name.replace("place_holder", "");
                        }
                        if ops[0] == Operator::Other("".to_string()) {
                            let node_idx = get_node_idx(graph, name);
                            if i == 0 {
                                if ops_copy.len() > 1 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !name.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else if ops_copy[i] == Operator::Equals {
                                if i <= ops_copy.len() - 2 && ops_copy.len() > 1 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !name.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else {
                                graph.update_edge(
                                    node_idx,
                                    parent_node_index,
                                    ops_copy[i].to_string(),
                                );
                            }
                        } else {
                            let mut unitary_name = ops[0].to_string();
                            let mut name_copy = name.to_string().clone();
                            remove_redundant_parens(&mut name_copy);
                            unitary_name.push_str("(".clone());
                            unitary_name.push_str(&name_copy.clone());
                            unitary_name.push_str(")".clone());
                            let node_idx = get_node_idx(graph, &mut unitary_name);
                            if i == 0 {
                                if ops_copy.len() > 1 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !unitary_name.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else if ops_copy[i] == Operator::Equals {
                                if i <= ops_copy.len() - 2 {
                                    if (ops_copy[i + 1].to_string() == "+"
                                        || ops_copy[i + 1].to_string() == "-")
                                        && !unitary_name.starts_with('-')
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "+".to_string(),
                                        );
                                    } else if ops_copy[i + 1].to_string() == "*"
                                        || ops_copy[i + 1].to_string() == "/"
                                    {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            "*".to_string(),
                                        );
                                    } else {
                                        graph.update_edge(
                                            node_idx,
                                            parent_node_index,
                                            ops_copy[i + 1].to_string(),
                                        );
                                    }
                                }
                            } else {
                                graph.update_edge(
                                    node_idx,
                                    parent_node_index,
                                    ops_copy[i].to_string(),
                                );
                            }
                        }
                        args[i].to_graph(graph);
                    }
                }
            }
        }
    }
}

/// The graph generation process adds additional parentheses to preserve operation order. This
/// function checks for the existence of those redundant parentheses.
pub fn contains_redundant_parens(string: &str) -> bool {
    let str_len = string.chars().count();
    if !string.starts_with('(') || string.chars().nth(str_len - 1) != Some(')') {
        return false;
    }
    let mut par_stack = VecDeque::new();
    par_stack.push_back("left_par");
    for i in 1..=str_len - 2 {
        if string.chars().nth(i) == Some('(') {
            par_stack.push_back("par");
        } else if string.chars().nth(i) == Some(')') {
            par_stack.pop_back();
        }
    }
    if !par_stack.is_empty() && par_stack[0] == "left_par" {
        return true;
    }
    false
}

/// Check if the current term's operators are all multiply or divide.
pub fn all_ops_are_mult_or_div(ops: Vec<Operator>) -> bool {
    for o in 1..=ops.len() - 1 {
        if ops[o] != Operator::Multiply && ops[o] != Operator::Divide {
            return false;
        }
    }
    true
}

///Switch the multiplication operator and the division operator
pub fn switch_mul_div(op: Operator) -> Operator {
    let switched_op: Operator = Operator::Other("".to_string());
    if op == Operator::Multiply {
        return Operator::Divide;
    }
    if op == Operator::Divide {
        return Operator::Multiply;
    }
    return switched_op;
}

///Switch the summation operator and the subtraction operator
pub fn switch_add_subt(op: Operator) -> Operator {
    let switched_op: Operator = Operator::Other("".to_string());
    if op == Operator::Add {
        return Operator::Subtract;
    }
    if op == Operator::Subtract {
        return Operator::Add;
    }
    return switched_op;
}

/// Check if the current term's operators are all add or subtract.
pub fn all_ops_are_add_or_subt(ops: Vec<Operator>) -> bool {
    for o in 1..=ops.len() - 1 {
        if ops[o] != Operator::Add && ops[o] != Operator::Subtract {
            return false;
        }
    }
    true
}

/// Check if the current term's operators contain multiply.
pub fn ops_contain_mult(ops: Vec<Operator>) -> bool {
    for o in 1..=ops.len() - 1 {
        if ops[o] == Operator::Multiply {
            return true;
        }
    }
    return false;
}

/// Check if the current term's operators contain multiple divisions, and the denominators contain
/// add and subtract and without unary operators
pub fn need_to_distribute_divs(ops: Vec<Operator>, args: Vec<Expr>) -> bool {
    let mut num_div: i32 = 0;
    let mut contain_add_subt_without_uop: bool = false;
    for o in 1..=ops.len() - 1 {
        if ops[o] == Operator::Divide {
            num_div = num_div + 1;
            if let Expr::Expression { ops, args, name } = &args[o] {
                if ops[0] == Operator::Other("".to_string()) && all_ops_are_add_or_subt(ops.clone())
                {
                    contain_add_subt_without_uop = true;
                }
            }
            if num_div > 1 && contain_add_subt_without_uop {
                return true;
            }
        }
    }
    return false;
}

/// Check if the current term's operators contain add or minus and without the unary operator.
pub fn need_to_distribute(ops: Vec<Operator>) -> bool {
    if ops[0] != Operator::Other("".to_string()) {
        return false;
    }
    for o in 1..=ops.len() - 1 {
        if ops[o] == Operator::Add || ops[o] == Operator::Subtract {
            return true;
        }
    }
    return false;
}

impl PreExp {
    pub fn group_expr(&mut self) {
        for arg in &mut self.args {
            if let Expr::Expression { .. } = arg {
                arg.group_expr();
            }
        }
    }

    pub fn collapse_expr(&mut self) {
        for arg in &mut self.args {
            if let Expr::Expression { .. } = arg {
                arg.collapse_expr();
            }
        }
    }

    fn distribute_expr(&mut self) {
        for arg in &mut self.args {
            if let Expr::Expression { .. } = arg {
                arg.distribute_expr();
            }
        }
    }

    pub fn set_name(&mut self) {
        for arg in &mut self.args {
            if let Expr::Expression { .. } = arg {
                arg.set_name();
            }
        }
    }

    pub fn to_graph(&mut self) -> MathExpressionGraph {
        let mut g = MathExpressionGraph::new();
        for arg in &mut self.args {
            if let Expr::Expression { .. } = arg {
                arg.to_graph(&mut g);
            }
        }
        g
    }
}

/// Remove redundant parentheses.
pub fn remove_redundant_parens(string: &mut String) -> &mut String {
    while contains_redundant_parens(string) {
        string.remove(string.len() - 1);
        string.remove(0);
    }
    *string = string.replace("()", "");
    string
}

/// Return the node index if it exists; if not, add a new node and return the node index.
pub fn get_node_idx(graph: &mut MathExpressionGraph, name: &mut String) -> NodeIndex {
    remove_redundant_parens(name);
    if name.contains("derivative") {
        *name = name.replace('/', ", ");
    }
    if graph.node_count() > 0 {
        for n in 0..=graph.node_count() - 1 {
            if let Some(x) = graph.raw_nodes().get(n) {
                if *name == x.weight {
                    if let Some(x) = graph.node_indices().nth(n) {
                        return x;
                    }
                }
            }
        }
    }

    graph.add_node(name.to_string())
}

/// Remove redundant mrow next to specific MathML elements. This function will likely be removed
/// once the img2mml pipeline is fixed.
pub fn remove_redundant_mrow(mml: String, key_word: String) -> String {
    let mut content = mml;
    let key_words_left = "<mrow>".to_string() + &*key_word.clone();
    let mut key_word_right = key_word.clone();
    key_word_right.insert(1, '/');
    let key_words_right = key_word_right.clone() + "</mrow>";
    let locs: Vec<_> = content
        .match_indices(&key_words_left)
        .map(|(i, _)| i)
        .collect();
    for loc in locs.iter().rev() {
        if content[loc + 1..].contains(&key_words_right) {
            let l = content[*loc..].find(&key_word_right).map(|i| i + *loc);
            if let Some(x) = l {
                if content.len() > (x + key_words_right.len())
                    && content[x..x + key_words_right.len()] == key_words_right
                {
                    content.replace_range(x..x + key_words_right.len(), key_word_right.as_str());
                    content.replace_range(*loc..*loc + key_words_left.len(), key_word.as_str());
                }
            }
        }
    }
    content
}

/// Remove redundant mrows in mathml because some mathml elements don't need mrow to wrap. This
/// function will likely be removed
/// once the img2mml pipeline is fixed.
pub fn remove_redundant_mrows(mathml_content: String) -> String {
    let mut content = mathml_content;
    content = content.replace("<mrow>", "(");
    content = content.replace("</mrow>", ")");
    let f = |b: &[u8]| -> Vec<u8> {
        let v = (0..)
            .zip(b)
            .scan(vec![], |a, (b, c)| {
                Some(match c {
                    40 => {
                        a.push(b);
                        None
                    }
                    41 => Some((a.pop()?, b)),
                    _ => None,
                })
            })
            .flatten()
            .collect::<Vec<_>>();
        for k in &v {
            if k.0 == 0 && k.1 == b.len() - 1 {
                return b[1..b.len() - 1].to_vec();
            }
            for l in &v {
                if l.0 == k.0 + 1 && l.1 == k.1 - 1 {
                    return [&b[..k.0], &b[l.0..k.1], &b[k.1 + 1..]].concat();
                }
            }
        }
        b.to_vec()
    };
    let g = |mut b: Vec<u8>| {
        while f(&b) != b {
            b = f(&b)
        }
        b
    };
    content = std::str::from_utf8(&g(content.bytes().collect()))
        .unwrap()
        .to_string();
    content = content.replace('(', "<mrow>");
    content = content.replace(')', "</mrow>");
    content = remove_redundant_mrow(content, "<mi>".to_string());
    content = remove_redundant_mrow(content, "<mo>".to_string());
    content = remove_redundant_mrow(content, "<mfrac>".to_string());
    content = remove_redundant_mrow(content, "<mover>".to_string());
    content
}

/// Preprocess the content prior to parsing.
pub fn preprocess_content(content_str: String) -> String {
    let mut pre_string = content_str;
    pre_string = pre_string.replace(' ', "");
    pre_string = pre_string.replace('\n', "");
    pre_string = pre_string.replace('\t', "");
    pre_string = pre_string.replace("<mo>(</mo><mi>t</mi><mo>)</mo>", "");
    pre_string = pre_string.replace("<mo>,</mo>", "");
    pre_string = pre_string.replace("<mo>(</mo>", "<mrow>");
    pre_string = pre_string.replace("<mo>)</mo>", "</mrow>");

    // Unicode to Symbol
    let unicode_locs: Vec<_> = pre_string.match_indices("&#").map(|(i, _)| i).collect();
    for ul in unicode_locs.iter().rev() {
        let loc = pre_string[*ul..].find('<').map(|i| i + ul);
        match loc {
            None => {}
            Some(x) => pre_string.insert(x, ';'),
        }
    }
    pre_string = html_escape::decode_html_entities(&pre_string).to_string();
    pre_string = pre_string.replace(
        &html_escape::decode_html_entities("&#x2212;").to_string(),
        "-",
    );
    pre_string = remove_redundant_mrows(pre_string);
    pre_string
}

/// Wrap mathml vectors by mrow as a single expression to process
pub fn wrap_math(math: Math) -> MathExpression {
    let mut math_vec = vec![];
    for con in math.content {
        math_vec.push(con);
    }

    Mrow(math_vec)
}

#[test]
fn test_to_expr() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    if let Expr::Expression { ops, args, .. } = &pre_exp.args[0] {
        assert_eq!(ops[0], Operator::Other("".to_string()));
        assert_eq!(ops[1], Operator::Add);
        assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
        assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
    }
}

#[test]
fn test_to_expr2() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Subtract),
        Mrow(vec![
            Mn("4".to_string()),
            Mi("c".to_string()),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };

    math_expression.to_expr(&mut pre_exp);
    pre_exp.ops.push(Operator::Other("root".to_string()));
    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(ops[2], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Number("4".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr3() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ])));
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Sqrt);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr4() {
    let math_expression = Mfrac(
        Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ])),
        Box::from(Mi("c".to_string())),
    );
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Divide);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_x) => {
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
                Expr::Expression { .. } => {}
            }
        }
    }
}

#[test]
fn test_to_expr5() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr6() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("h".to_string())));
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(ops[2], Operator::Multiply);
                    assert_eq!(ops[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                    assert_eq!(args[3], Expr::Atom(Atom::Identifier("e".to_string())));
                }
            }
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("f".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("g".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr7() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, name } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(name, "(a+b*c)");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, name } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(name, "b*c");
                }
            }
        }
    }
}

#[test]
fn test_to_expr8() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, name } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("h".to_string())));
            assert_eq!(name, "(a+b*c*d/e-f*g-h)");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, name } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(ops[2], Operator::Multiply);
                    assert_eq!(ops[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                    assert_eq!(args[3], Expr::Atom(Atom::Identifier("e".to_string())));
                    assert_eq!(name, "b*c*d/e");
                }
            }
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, name } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("f".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("g".to_string())));
                    assert_eq!(name, "f*g");
                }
            }
        }
    }
}

#[test]
fn test_to_expr9() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, name } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(name, "(a+b*(c-d))");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, name } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(name, "b*(c-d)");
                    match &args[1] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, name } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Subtract);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                            assert_eq!(name, "(c-d)");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_to_expr10() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("a".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr11() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ])));

    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr12() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr13() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("a".to_string()),
        Mo(Operator::Divide),
        Mi("d".to_string()),
        Mo(Operator::Subtract),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr14() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr15() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Subtract),
        Msqrt(Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]))),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.set_name();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr16() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ])));
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr17() {
    let math_expression = Mrow(vec![
        Mi("s".to_string()),
        Mo(Operator::Equals),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr18() {
    let math_expression = Mrow(vec![
        Mi("s".to_string()),
        Mo(Operator::Equals),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Subtract),
        Msqrt(Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
            Mo(Operator::Multiply),
            Mrow(vec![
                Mi("a".to_string()),
                Mo(Operator::Subtract),
                Mi("b".to_string()),
            ]),
        ]))),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr19() {
    use crate::parsing::parse;
    let input = "tests/sir.xml";
    let contents = std::fs::read_to_string(input)
        .unwrap_or_else(|_| panic!("{}", "Unable to read file {input}!"));
    let (_, mut math) =
        parse(&contents).unwrap_or_else(|_| panic!("{}", "Unable to parse file {input}!"));
    math.normalize();
    let _g = &mut math.content[0].clone().to_graph();
}

#[test]
fn test_to_expr20() {
    let math_expression = Mrow(vec![
        Mi("s".to_string()),
        Mo(Operator::Equals),
        Mfrac(
            Box::from(Mrow(vec![
                Mi("a".to_string()),
                Mo(Operator::Add),
                Mi("b".to_string()),
            ])),
            Box::from(Mrow(vec![
                Mi("a".to_string()),
                Mo(Operator::Multiply),
                Mi("c".to_string()),
                Mi("d".to_string()),
                Msqrt(Box::from(Mrow(vec![
                    Mi("a".to_string()),
                    Mo(Operator::Add),
                    Mi("d".to_string()),
                ]))),
            ])),
        ),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr21() {
    let math_expression = Msup(
        Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ])),
        Box::from(Mi("c".to_string())),
    );
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Other("^".to_string()));
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_x) => {
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
                Expr::Expression { .. } => {}
            }
        }
    }
}

#[test]
fn test_to_expr22() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Msup(
            Box::from(Mrow(vec![
                Mi("a".to_string()),
                Mo(Operator::Add),
                Mi("b".to_string()),
            ])),
            Box::from(Mrow(vec![
                Mi("c".to_string()),
                Mo(Operator::Add),
                Mi("d".to_string()),
            ])),
        ),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr23() {
    let math_expression = Mrow(vec![Msubsup(
        Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ])),
        Box::from(Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("d".to_string()),
        ])),
        Box::from(Mi("c".to_string())),
    )]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr24() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Add),
        Mi("c".to_string()),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr25() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr26() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Add),
        Mi("d".to_string()),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr27() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr28() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr29() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Add),
        Msup(
            Box::from(Mrow(vec![
                Mo(Operator::Subtract),
                Mi("a".to_string()),
                Mo(Operator::Add),
                Mi("b".to_string()),
            ])),
            Box::from(Mrow(vec![
                Mi("c".to_string()),
                Mo(Operator::Add),
                Mi("d".to_string()),
            ])),
        ),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr30() {
    use crate::parsing::parse;
    let input = "tests/seir_eq1.xml";
    let mut contents = std::fs::read_to_string(input)
        .unwrap_or_else(|_| panic!("{}", "Unable to read file {input}!"));
    contents = preprocess_content(contents);
    let (_, mut math) =
        parse(&contents).unwrap_or_else(|_| panic!("{}", "Unable to parse file {input}!"));
    math.normalize();
    let mut math_vec = vec![];
    for con in math.content {
        math_vec.push(con);
    }
    let new_math = Mrow(math_vec);
    let _g = new_math.to_graph();
}

#[test]
fn test_to_expr32() {
    use crate::parsing::parse;
    let input = "tests/seirdv_eq7.xml";
    let mut contents = std::fs::read_to_string(input)
        .unwrap_or_else(|_| panic!("{}", "Unable to read file {input}!"));
    contents = preprocess_content(contents);
    let (_, mut math) =
        parse(&contents).unwrap_or_else(|_| panic!("{}", "Unable to parse file {input}!"));
    math.normalize();
    let new_math = wrap_math(math);
    let _g = new_math.to_graph();
}

#[test]
fn test_to_expr33() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("b".to_string()),
            Mo(Operator::Add),
            Mi("c".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr34() {
    let math_expression = Mrow(vec![
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]),
        Mo(Operator::Divide),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Add),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Divide);

            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }

            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr35() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mrow(vec![
            Mi("b".to_string()),
            Mo(Operator::Subtract),
            Mi("c".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Subtract);
            assert_eq!(ops[2], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            assert_eq!(args[2], Expr::Atom(Atom::Identifier("c".to_string())));
        }
    }
}

#[test]
fn test_to_expr36() {
    let math_expression = Mrow(vec![
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
        Mo(Operator::Subtract),
        Mi("c".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Subtract);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            assert_eq!(args[2], Expr::Atom(Atom::Identifier("c".to_string())));
        }
    }
}

#[test]
fn test_to_expr37() {
    let math_expression = Mrow(vec![
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]),
        Mo(Operator::Subtract),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Add),
            Mi("d".to_string()),
        ]),
        Mo(Operator::Add),
        Mi("e".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Add);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(ops[3], Operator::Subtract);
            assert_eq!(ops[4], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            assert_eq!(args[2], Expr::Atom(Atom::Identifier("c".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("d".to_string())));
            assert_eq!(args[4], Expr::Atom(Atom::Identifier("e".to_string())));
        }
    }
}

#[test]
fn test_to_expr38() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mrow(vec![
            Mrow(vec![
                Mi("b".to_string()),
                Mo(Operator::Subtract),
                Mi("c".to_string()),
            ]),
            Mo(Operator::Add),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Subtract);
            assert_eq!(ops[2], Operator::Add);
            assert_eq!(ops[3], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            assert_eq!(args[2], Expr::Atom(Atom::Identifier("c".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("d".to_string())));
        }
    }
}

#[test]
fn test_to_expr39() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Divide),
        Mrow(vec![
            Mrow(vec![
                Mi("b".to_string()),
                Mo(Operator::Divide),
                Mi("c".to_string()),
            ]),
            Mo(Operator::Multiply),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Divide);
            assert_eq!(ops[2], Operator::Multiply);
            assert_eq!(ops[3], Operator::Divide);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            assert_eq!(args[2], Expr::Atom(Atom::Identifier("c".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("d".to_string())));
        }
    }
}

#[test]
fn test_to_expr40() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Divide),
        Mrow(vec![
            Mi("b".to_string()),
            Mo(Operator::Add),
            Mi("c".to_string()),
        ]),
        Mo(Operator::Divide),
        Mrow(vec![
            Mi("d".to_string()),
            Mo(Operator::Subtract),
            Mi("e".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Divide);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Subtract);
                    assert_eq!(ops[2], Operator::Add);
                    assert_eq!(ops[3], Operator::Subtract);
                    match &args[0] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Multiply);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                        }
                    }
                    match &args[1] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Multiply);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("e".to_string())));
                        }
                    }
                    match &args[2] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Multiply);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                        }
                    }
                    match &args[3] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Multiply);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("e".to_string())));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_to_expr41() {
    let math_expression = Mrow(vec![
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Subtract);
            assert_eq!(ops[2], Operator::Add);
            assert_eq!(ops[3], Operator::Subtract);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                }
            }
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
            }
            match &args[3] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr42() {
    let math_expression = Mrow(vec![
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mrow(vec![
                Mi("b".to_string()),
                Mo(Operator::Add),
                Mi("c".to_string()),
            ]),
        ]),
        Mo(Operator::Divide),
        Mrow(vec![
            Mi("f".to_string()),
            Mo(Operator::Add),
            Mi("g".to_string()),
        ]),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("d".to_string()),
            Mo(Operator::Subtract),
            Mi("e".to_string()),
        ]),
        Mo(Operator::Divide),
        Mrow(vec![
            Mi("h".to_string()),
            Mo(Operator::Subtract),
            Mi("i".to_string()),
        ]),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("j".to_string()),
            Mo(Operator::Divide),
            Mi("k".to_string()),
        ]),
        Mo(Operator::Add),
        Mi("l".to_string()),
    ]);
    let mut pre_exp = PreExp {
        ops: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.ops.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.distribute_expr();
    pre_exp.group_expr();
    pre_exp.collapse_expr();
    pre_exp.set_name();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { ops, args, .. } => {
            assert_eq!(ops[0], Operator::Other("".to_string()));
            assert_eq!(ops[1], Operator::Subtract);
            assert_eq!(ops[2], Operator::Subtract);
            assert_eq!(ops[3], Operator::Add);
            assert_eq!(ops[4], Operator::Subtract);
            assert_eq!(ops[5], Operator::Add);
            assert_eq!(ops[6], Operator::Add);
            assert_eq!(args[6], Expr::Atom(Atom::Identifier("l".to_string())));
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(ops[2], Operator::Multiply);
                    assert_eq!(ops[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("j".to_string())));
                    match &args[3] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Subtract);
                            assert_eq!(ops[2], Operator::Add);
                            assert_eq!(ops[3], Operator::Subtract);
                            match &args[0] {
                                Expr::Atom(_) => {}
                                Expr::Expression { ops, args, .. } => {
                                    assert_eq!(ops[0], Operator::Other("".to_string()));
                                    assert_eq!(ops[1], Operator::Multiply);
                                    assert_eq!(ops[2], Operator::Multiply);
                                    assert_eq!(
                                        args[0],
                                        Expr::Atom(Atom::Identifier("f".to_string()))
                                    );
                                    assert_eq!(
                                        args[1],
                                        Expr::Atom(Atom::Identifier("h".to_string()))
                                    );
                                    assert_eq!(
                                        args[2],
                                        Expr::Atom(Atom::Identifier("k".to_string()))
                                    );
                                }
                            }
                            match &args[3] {
                                Expr::Atom(_) => {}
                                Expr::Expression { ops, args, .. } => {
                                    assert_eq!(ops[0], Operator::Other("".to_string()));
                                    assert_eq!(ops[1], Operator::Multiply);
                                    assert_eq!(ops[2], Operator::Multiply);
                                    assert_eq!(
                                        args[0],
                                        Expr::Atom(Atom::Identifier("g".to_string()))
                                    );
                                    assert_eq!(
                                        args[1],
                                        Expr::Atom(Atom::Identifier("i".to_string()))
                                    );
                                    assert_eq!(
                                        args[2],
                                        Expr::Atom(Atom::Identifier("k".to_string()))
                                    );
                                }
                            }
                        }
                    }
                }
            }
            match &args[5] {
                Expr::Atom(_) => {}
                Expr::Expression { ops, args, .. } => {
                    assert_eq!(ops[0], Operator::Other("".to_string()));
                    assert_eq!(ops[1], Operator::Multiply);
                    assert_eq!(ops[2], Operator::Multiply);
                    assert_eq!(ops[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("e".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("j".to_string())));
                    match &args[3] {
                        Expr::Atom(_) => {}
                        Expr::Expression { ops, args, .. } => {
                            assert_eq!(ops[0], Operator::Other("".to_string()));
                            assert_eq!(ops[1], Operator::Subtract);
                            assert_eq!(ops[2], Operator::Add);
                            assert_eq!(ops[3], Operator::Subtract);
                            match &args[0] {
                                Expr::Atom(_) => {}
                                Expr::Expression { ops, args, .. } => {
                                    assert_eq!(ops[0], Operator::Other("".to_string()));
                                    assert_eq!(ops[1], Operator::Multiply);
                                    assert_eq!(ops[2], Operator::Multiply);
                                    assert_eq!(
                                        args[0],
                                        Expr::Atom(Atom::Identifier("f".to_string()))
                                    );
                                    assert_eq!(
                                        args[1],
                                        Expr::Atom(Atom::Identifier("h".to_string()))
                                    );
                                    assert_eq!(
                                        args[2],
                                        Expr::Atom(Atom::Identifier("k".to_string()))
                                    );
                                }
                            }
                            match &args[3] {
                                Expr::Atom(_) => {}
                                Expr::Expression { ops, args, .. } => {
                                    assert_eq!(ops[0], Operator::Other("".to_string()));
                                    assert_eq!(ops[1], Operator::Multiply);
                                    assert_eq!(ops[2], Operator::Multiply);
                                    assert_eq!(
                                        args[0],
                                        Expr::Atom(Atom::Identifier("g".to_string()))
                                    );
                                    assert_eq!(
                                        args[1],
                                        Expr::Atom(Atom::Identifier("i".to_string()))
                                    );
                                    assert_eq!(
                                        args[2],
                                        Expr::Atom(Atom::Identifier("k".to_string()))
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
