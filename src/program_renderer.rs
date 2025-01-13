use crate::variable::{
    Expression, FunctionCall, OutlineStatement, TypeAnnotation, VariableDeclaration,
};
use fake::faker::company::en::BuzzwordTail;
use fake::Fake;
use std::collections::{HashMap, HashSet};
use rand::Rng;
use rand::rngs::StdRng;

pub fn render_program(outline: &Vec<OutlineStatement>, rng: &mut StdRng) -> Vec<String> {
    let mut definitions: Vec<String> = vec![];
    let mut program: Vec<String> = vec![];

    let function_names_and_signatures = extract_function_names_and_signatures(outline);
    let function_names_by_signatures: HashMap<Vec<TypeAnnotation>, String> =
        function_names_and_signatures
            .iter()
            .map(|(signature, name)| (signature.clone(), name.clone()))
            .collect();

    for function_definition in function_names_and_signatures {
        definitions.push(render_function_definition(
            function_definition.0,
            function_definition.1.clone(),
        ));
    }

    for statement in outline {
        match statement {
            OutlineStatement::VariableDeclaration(declaration) => {
                program.push(render_variable_declaration(declaration.clone(), rng));
            }
            OutlineStatement::FunctionCall(call) => {
                program.push(render_function_call(
                    call.clone(),
                    &function_names_by_signatures,
                    rng
                ));
            }
            OutlineStatement::PrintVariables(variables) => program.push(render_print_variables(
                variables.iter().map(|v| v.left_info.name.clone()).collect(),
            )),
        }
    }

    format!(
        "{}\n\n fn main() {{\n{}\n}}",
        definitions.join("\n"),
        program.join("\n")
    )
    .split("\n")
    .map(|s| s.to_string())
    .collect()
}

fn extract_function_names_and_signatures(
    outline: &Vec<OutlineStatement>,
) -> Vec<(Vec<TypeAnnotation>, String)> {
    // add all function definitions
    let function_calls: Vec<FunctionCall> = outline
        .iter()
        .cloned()
        // pattern match to FunctionCall
        .filter_map(|statement| match statement {
            OutlineStatement::FunctionCall(call) => Some(call),
            _ => None,
        })
        .collect();

    let signatures: HashSet<Vec<TypeAnnotation>> = function_calls
        .iter()
        .map(|call| {
            call.arguments
                .iter()
                .map(|arg| arg.expr_type())
                .collect::<Vec<_>>()
        })
        .collect();

    let function_names_and_signatures = signatures
        // zip with indices
        .into_iter()
        .enumerate()
        // map to function name
        .map(|(index, signature)| {
            let function_name = format!("f{}", index + 1);
            (signature, function_name)
        })
        .collect::<Vec<(Vec<TypeAnnotation>, String)>>();
    function_names_and_signatures
}

fn render_function_definition(
    argument_types: Vec<TypeAnnotation>,
    function_name: String,
) -> String {
    let mut rendered_arguments = vec![];
    let mut suffix: i32 = 1;
    for argument_type in argument_types {
        rendered_arguments.push(format!(
            "arg{}: {}",
            suffix,
            render_type_annotation(argument_type)
        ));
        suffix += 1;
    }
    format!(
        "fn {}({}) {{}}",
        function_name,
        rendered_arguments.join(", ")
    )
}

fn render_function_call(
    call: FunctionCall,
    function_names_by_signatures: &HashMap<Vec<TypeAnnotation>, String>,
    rng: &mut StdRng
) -> String {
    let mut rendered_arguments = vec![];
    let call_argument_types: Vec<TypeAnnotation> =
        call.arguments.iter().map(|arg| arg.expr_type()).collect();
    let function_name = function_names_by_signatures
        .get(&call_argument_types)
        .unwrap();
    for argument in call.arguments {
        rendered_arguments.push(render_expression(argument, rng));
    }

    format!("{}({});", function_name, rendered_arguments.join(", "))
}

fn render_print_variables(names: Vec<String>) -> String {
    format!(
        "println!(\"{}\", {});",
        "{:?} ".repeat(names.len()),
        names.join(", ")
    )
}

fn render_type_annotation(type_annotation: TypeAnnotation) -> String {
    match type_annotation {
        TypeAnnotation::Reference(t) => format!("&{}", render_type_annotation(*t)),
        TypeAnnotation::Vector(t) => format!("Vec<{}>", render_type_annotation(*t)),
        TypeAnnotation::Int32 => String::from("i32"),
        TypeAnnotation::String => String::from("String"),
        TypeAnnotation::Slice => String::from("str"),
    }
}

fn render_expression(expression: Expression, rng: &mut StdRng) -> String {
    match expression {
        Expression::VectorLiteral { contents } => {
            let mut rendered_contents = vec![];
            for content in contents {
                rendered_contents.push(render_expression(content, rng));
            }
            format!("vec![{}]", rendered_contents.join(", "))
        }
        Expression::IntLiteral { .. } => {
            format!("{}", rng.gen_range(1..100))
        }
        Expression::StringFromLiteral { .. } => {
            format! {r#"String::from("{}")"#, BuzzwordTail().fake::<String>().to_lowercase()}
        }
        Expression::Name { name } => {
            format!("{}", name.name())
        }
        Expression::Reference {
            expression,
            is_mutable,
        } => {
            format!(
                "&{}{}",
                if is_mutable { "mut " } else { "" },
                render_expression(*expression, rng)
            )
        }
        Expression::Dereference { expression } => {
            format!("*{}", render_expression(*expression, rng))
        }
        Expression::IndexAccess { expression, index } => {
            format!("{}[{}]", render_expression(*expression, rng), index)
        }
        Expression::SliceAccess {
            expression,
            start_index,
            end_index,
        } => {
            format!(
                "{}[{}..{}]",
                render_expression(*expression, rng),
                start_index
                    .map(|u| format!("{}", u))
                    .unwrap_or(String::from("")),
                end_index
                    .map(|u| format!("{}", u))
                    .unwrap_or(String::from(""))
            )
        }
    }
}

fn render_variable_declaration(variable_declaration: VariableDeclaration, rng: &mut StdRng) -> String {
    format!(
        "let {}{} : {} = {};",
        if variable_declaration.left_info.is_mutable {
            "mut "
        } else {
            ""
        },
        variable_declaration.left_info.name,
        render_type_annotation(variable_declaration.right_expression.expr_type()),
        render_expression(variable_declaration.right_expression, rng)
    )
}
