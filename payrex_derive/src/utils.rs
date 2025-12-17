use syn::{GenericArgument, PathArguments, Type};

pub(crate) fn is_type(ty: &Type, name: &str) -> bool {
    if let syn::Type::Path(p) = ty
        && let Some(seg) = p.path.segments.last()
    {
        return seg.ident == name;
    }
    false
}

pub(crate) fn is_type_deep(ty: &Type, name: &str) -> bool {
    let path = match ty {
        Type::Path(type_path) if type_path.qself.is_none() => &type_path.path,
        _ => return false,
    };

    if let Some(segment) = path.segments.last() {
        return segment.ident == name;
    }

    false
}

pub(crate) fn get_option_inner(ty: &Type) -> Option<&Type> {
    let path = match ty {
        Type::Path(type_path) if type_path.qself.is_none() => &type_path.path,
        _ => return None,
    };

    let segment = path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(args) => &args.args,
        _ => return None,
    };

    if args.len() != 1 {
        return None;
    }

    match args.first()? {
        GenericArgument::Type(inner_type) => Some(inner_type),
        _ => None,
    }
}
