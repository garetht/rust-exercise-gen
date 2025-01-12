use crate::namer::create_name;
use crate::variable::TypeAnnotation::{Reference, Vector};
use crate::weighted_choices::WeightedChoices;
use rand::prelude::{SliceRandom, StdRng};
use rand::Rng;
use std::cmp::min;
use std::collections::HashMap;

pub struct AvailableVariables {
    variables: Vec<VariableDeclaration>,
}

impl AvailableVariables {
    pub fn new(variables: Vec<VariableDeclaration>) -> Self {
        Self { variables }
    }

    fn count_by_type_class(&self) -> HashMap<TypeAnnotation, u8> {
        self.variables.iter().fold(HashMap::new(), |mut acc, var| {
            let annotation = match var.expr_type() {
                Reference(inner) => *inner,
                t => t,
            };
            let count = acc.get(&annotation).unwrap_or(&0) + 1;
            acc.insert(annotation, count);
            acc
        })
    }

    pub fn next_index_for_type(&self, type_annotation: &TypeAnnotation) -> u8 {
        let annotation = match type_annotation {
            Reference(inner) => &*inner,
            t => t,
        };
        self.count_by_type_class()
            .get(annotation)
            .unwrap_or(&0)
            .clone()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MemoryType {
    Heap,
    Stack,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Position {
    FunctionCall,
    Assignment,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeAnnotation {
    Reference(Box<TypeAnnotation>),
    Vector(Box<TypeAnnotation>),
    Int32,
    String,
    Slice
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub is_mutable: bool,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Expression {
    // literal, e.g. vec![1, 2, 3]
    VectorLiteral {
        contents: Vec<Expression>,
    },
    // Integer literal
    IntLiteral {},
    // String from literal, i.e. String::from(...)
    StringFromLiteral {},
    // just a name, like x
    Name {
        name: Box<VariableDeclaration>,
    },
    // reference, e.g. &x
    Reference {
        expression: Box<Expression>,
        is_mutable: bool,
    },
    // dereference, e.g. *x
    Dereference {
        expression: Box<Expression>,
    },
    // index access, e.g. x[2]
    IndexAccess {
        expression: Box<Expression>,
        index: u8,
    },
    SliceAccess {
        expression: Box<Expression>,
        start_index: Option<u8>,
        end_index: Option<u8>,
    },
}

// if variable is mutable, 50% chance that assignment does not
// have a declaration
impl Expression {
    pub fn expr_type(&self) -> TypeAnnotation {
        match self {
            Expression::VectorLiteral { contents } => {
                Vector(Box::new(contents.first().unwrap().expr_type()))
            }
            Expression::IntLiteral { .. } => TypeAnnotation::Int32,
            Expression::StringFromLiteral { .. } => TypeAnnotation::String,
            Expression::Name { name } => name.right_expression.expr_type(),
            Expression::Reference { expression, .. } => Reference(Box::new(expression.expr_type())),
            Expression::Dereference { expression } => match expression.expr_type() {
                Reference(annotation) => *annotation,
                _ => panic!("Cannot dereference non-reference"),
            },
            Expression::IndexAccess { expression, .. } => match expression.expr_type() {
                Vector(annotation) => *annotation,
                _ => panic!("Cannot index non-vector {:?}", expression),
            },
            Expression::SliceAccess { .. } => TypeAnnotation::Slice
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub left_info: VariableInfo,
    pub right_expression: Expression,
}

impl VariableDeclaration {
    pub fn name(&self) -> String {
        self.left_info.name.clone()
    }

    pub fn expr_type(&self) -> TypeAnnotation {
        self.right_expression.expr_type()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum OutlineStatement {
    VariableDeclaration(VariableDeclaration),
    FunctionCall(FunctionCall),
    PrintVariables(Vec<VariableDeclaration>),
}

/**
    Either move by giving it to another variable assignment, or move by
    passing it to a function call
**/
pub fn rand_move(
    mut rng: &mut StdRng,
    available_variables: &AvailableVariables,
) -> OutlineStatement {
    let variable_to_move = available_variables.variables.choose(&mut rng).unwrap();

    let should_dereference = if let Expression::Reference { .. } = &variable_to_move.right_expression {
        true
    } else {
        false
    };

    let move_expr = Expression::Name {
        name: Box::new(variable_to_move.clone()),
    };
    let full_move_expr = if should_dereference {
        Expression::Dereference {
            expression: Box::new(move_expr),
        }
    } else {
        move_expr
    };

    create_call_or_assignment(&mut rng, available_variables, full_move_expr)
}

pub fn rand_borrow(
    mut rng: &mut StdRng,
    available_variables: &AvailableVariables,
) -> OutlineStatement {
    let variable_to_borrow = available_variables.variables.choose(&mut rng).unwrap();

    let is_mutable_borrow = if variable_to_borrow.left_info.is_mutable {
        rng.gen_bool(0.5)
    } else {
        rng.gen_bool(0.05)
    };

    let base_expr_to_borrow = Expression::Name {
        name: Box::new(variable_to_borrow.clone()),
    };
    let reference_expr_to_borrow = Expression::Reference {
        expression: Box::new(base_expr_to_borrow.clone()),
        is_mutable: is_mutable_borrow,
    };
    let borrow_type = variable_to_borrow.expr_type();
    let full_borrow_expr = if let Vector { .. } = borrow_type {
        if rng.gen_bool(0.5) {
            Expression::Reference {
                expression: Box::new(Expression::IndexAccess {
                    expression: Box::new(base_expr_to_borrow),
                    index: 0,
                }),
                is_mutable: is_mutable_borrow,
            }
        } else {
           reference_expr_to_borrow
        }
    } else if let TypeAnnotation::String { .. } = borrow_type {
        if rng.gen_bool(0.5) {
            Expression::Reference {
                expression: Box::new(Expression::SliceAccess {
                    expression: Box::new(base_expr_to_borrow),
                    start_index: None,
                    end_index: None
                }),
                is_mutable: is_mutable_borrow,
            }
        } else {
            reference_expr_to_borrow
        }
    } else {
        reference_expr_to_borrow
    };

    create_call_or_assignment(&mut rng, available_variables, full_borrow_expr)
}

fn create_call_or_assignment(rng: &mut &mut StdRng, available_variables: &AvailableVariables, full_borrow_expr: Expression) -> OutlineStatement {
    if rng.gen_bool(0.6) {
        OutlineStatement::FunctionCall(FunctionCall {
            arguments: vec![full_borrow_expr],
        })
    } else {
        let type_ann = full_borrow_expr.expr_type();
        OutlineStatement::VariableDeclaration(VariableDeclaration {
            left_info: VariableInfo {
                name: create_name(
                    &type_ann,
                    available_variables.next_index_for_type(&type_ann),
                ),
                is_mutable: false,
            },
            right_expression: full_borrow_expr,
        })
    }
}

pub fn rand_read(
    mut rng: &mut StdRng,
    available_variables: &AvailableVariables,
    must_read_all: bool,
) -> OutlineStatement {
    let weighted_number_of_variables = WeightedChoices::<u8>::exponential_decreasing(min(
        2,
        available_variables.variables.len() as u8,
    ));

    let number_to_print = if must_read_all {
        available_variables.variables.len() as u8
    } else {
        weighted_number_of_variables.choose(rng).unwrap()
    };

    let mut variables = available_variables.variables.clone();
    variables.shuffle(&mut rng);

    let variables = variables
        .iter()
        .cloned()
        .take(number_to_print.into())
        .collect();
    OutlineStatement::PrintVariables(variables)
}

pub fn rand_initialize_variable(
    rng: &mut StdRng,
    available_variables: AvailableVariables,
    memory_type: &MemoryType,
) -> VariableDeclaration {
    let is_vector = rng.gen_bool(0.5);
    let is_mutable = rng.gen_bool(0.5);

    match is_vector {
        true => {
            let expression = Expression::VectorLiteral {
                contents: match memory_type {
                    MemoryType::Heap => {
                        vec![Expression::StringFromLiteral {}; 2]
                    }
                    MemoryType::Stack => {
                        vec![Expression::IntLiteral {}; 2]
                    }
                },
            };
            let type_ann = expression.expr_type();
            VariableDeclaration {
                left_info: VariableInfo {
                    name: create_name(
                        &type_ann,
                        available_variables.next_index_for_type(&type_ann),
                    ),
                    is_mutable,
                },
                right_expression: expression,
            }
        }
        false => {
            let expression = match memory_type {
                MemoryType::Heap => Expression::StringFromLiteral {},
                MemoryType::Stack => Expression::IntLiteral {},
            };
            let type_ann = expression.expr_type();
            VariableDeclaration {
                left_info: VariableInfo {
                    name: create_name(
                        &type_ann,
                        available_variables.next_index_for_type(&type_ann),
                    ),
                    is_mutable,
                },
                right_expression: expression,
            }
        }
    }
}
