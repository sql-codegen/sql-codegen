use sqlparser::ast::ObjectName;

pub fn object_name_to_string(object_name: &ObjectName) -> String {
    let ObjectName(ident) = object_name;
    ident[0].value.clone()
}
