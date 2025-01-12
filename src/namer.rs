use crate::variable::TypeAnnotation;

fn create_name_for_type(type_annotation: &TypeAnnotation) -> String {
    match type_annotation {
        TypeAnnotation::Reference(inner) => create_name_for_type(inner),
        TypeAnnotation::Vector(_) => String::from("vec"),
        TypeAnnotation::Int32 => String::from("int"),
        TypeAnnotation::String => String::from("s"),
        TypeAnnotation::Slice => String::from("slice")
    }
}

pub fn create_name(type_annotation: &TypeAnnotation, next_index: u8) -> String {
    let name = match type_annotation {
        TypeAnnotation::Reference(inner) => create_name_for_type(inner),
        t => create_name_for_type(t)
    };

    if next_index > 0 {
        format!("{}{}", name, next_index + 1)
    } else {
        format!("{}", name)
    }
}

