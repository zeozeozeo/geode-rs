use pest::Parser;
use pest_derive::Parser as PestParser;
use std::path::Path;

use crate::ast::*;
use crate::error::{ParseError, Result};

#[derive(PestParser)]
#[grammar = "broma.pest"]
pub struct BromaParser;

#[derive(Default)]
struct ScratchData {
    is_class: bool,
    wip_class: Class,
    wip_fn_proto: FunctionProto,
    wip_field: Field,
    wip_bind: PlatformNumber,
    wip_type: Type,
    wip_mem_fn_proto: MemberFunctionProto,
    wip_attributes: Attributes,
    wip_fn_body: String,
    wip_has_explicit_inline: bool,
    wip_platform_block: Option<Platform>,
    wip_import_platform: Platform,
    field_id_counter: usize,
}

impl ScratchData {
    fn next_field_id(&mut self) -> usize {
        let id = self.field_id_counter;
        self.field_id_counter += 1;
        id
    }
}

pub fn parse_str(input: &str) -> Result<Root> {
    let pair = BromaParser::parse(Rule::root, input)?
        .next()
        .ok_or_else(|| ParseError::PestError("Empty input".to_string()))?;

    let mut root = Root::default();
    let mut scratch = ScratchData::default();

    parse_root(pair, &mut root, &mut scratch)?;

    post_process(&mut root);
    Ok(root)
}

pub fn parse_file(path: &Path) -> Result<Root> {
    let input = std::fs::read_to_string(path)?;
    parse_str(&input)
}

fn parse_root(
    pair: pest::iterators::Pair<Rule>,
    root: &mut Root,
    scratch: &mut ScratchData,
) -> Result<()> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::import_expr => {
                root.headers.push(parse_import(inner, scratch)?);
            }
            Rule::include_expr => {
                root.headers.push(parse_include(inner)?);
            }
            Rule::attribute => {
                parse_attribute(&inner, scratch)?;
            }
            Rule::class_statement => {
                root.classes.push(parse_class(inner, scratch)?);
            }
            Rule::function => {
                root.functions.push(parse_function(inner, scratch)?);
            }
            Rule::EOI | Rule::sep => {}
            _ => {}
        }
    }
    Ok(())
}

fn parse_import(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<Header> {
    scratch.wip_import_platform = Platform::All;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::platform => {
                scratch.wip_import_platform = str_to_platform(inner.as_str());
            }
            Rule::import_name => {
                return Ok(Header {
                    name: inner.as_str().trim().to_string(),
                    platform: scratch.wip_import_platform,
                });
            }
            _ => {}
        }
    }

    Ok(Header {
        name: String::new(),
        platform: scratch.wip_import_platform,
    })
}

fn parse_include(pair: pest::iterators::Pair<Rule>) -> Result<Header> {
    let name = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::include_name)
        .map(|p| p.as_str().trim().to_string())
        .unwrap_or_default();

    Ok(Header {
        name,
        platform: Platform::All,
    })
}

fn parse_class(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<Class> {
    scratch.is_class = true;
    scratch.wip_class = Class::default();
    scratch.wip_class.attributes = std::mem::take(&mut scratch.wip_attributes);

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::qualified => {
                let name = inner.as_str().to_string();
                if scratch.wip_class.name.is_empty() {
                    scratch.wip_class.name = name;
                } else {
                    if name == scratch.wip_class.name {
                        let (line, column) = inner.line_col();
                        return Err(ParseError::SelfInheritance { name, line, column });
                    }
                    scratch.wip_class.superclasses.push(name.clone());
                    scratch.wip_class.attributes.depends.push(name);
                }
            }
            Rule::field => {
                if let Some(field) = parse_field(inner, scratch)? {
                    scratch.wip_class.fields.push(field);
                }
            }
            Rule::platform_expr => {
                let fields = parse_platform_expr(inner, scratch)?;
                scratch.wip_class.fields.extend(fields);
            }
            _ => {}
        }
    }

    Ok(std::mem::take(&mut scratch.wip_class))
}

fn parse_platform_expr(
    pair: pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<Vec<Field>> {
    let mut platform = Platform::None;
    let mut fields = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::platform => {
                platform |= str_to_platform(inner.as_str());
            }
            Rule::field => {
                scratch.wip_platform_block = Some(platform);
                if let Some(field) = parse_field(inner, scratch)? {
                    fields.push(field);
                }
                scratch.wip_platform_block = None;
            }
            _ => {}
        }
    }

    Ok(fields)
}

fn parse_field(
    pair: pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<Option<Field>> {
    scratch.wip_field = Field::default();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::inline_expr => {
                parse_inline_expr(inner, scratch)?;
                scratch.wip_field.parent = scratch.wip_class.name.clone();
                scratch.wip_field.field_id = scratch.next_field_id();
                return Ok(Some(std::mem::take(&mut scratch.wip_field)));
            }
            Rule::pad_expr => {
                parse_pad_expr(inner, scratch)?;
                scratch.wip_field.parent = scratch.wip_class.name.clone();
                scratch.wip_field.field_id = scratch.next_field_id();
                return Ok(Some(std::mem::take(&mut scratch.wip_field)));
            }
            Rule::member_expr => {
                parse_member_expr(inner, scratch)?;
                scratch.wip_field.parent = scratch.wip_class.name.clone();
                scratch.wip_field.field_id = scratch.next_field_id();
                return Ok(Some(std::mem::take(&mut scratch.wip_field)));
            }
            Rule::bind_expr => {
                parse_bind_expr(inner, scratch)?;
                scratch.wip_field.parent = scratch.wip_class.name.clone();
                scratch.wip_field.field_id = scratch.next_field_id();
                return Ok(Some(std::mem::take(&mut scratch.wip_field)));
            }
            _ => {}
        }
    }

    Ok(None)
}

fn parse_inline_expr(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    let inner_str = pair.as_str();
    scratch.wip_field.inner = FieldInner::Inline(InlineField {
        inner: inner_str.to_string(),
    });
    Ok(())
}

fn parse_pad_expr(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    scratch.wip_bind = PlatformNumber::new();

    if let Some(platform) = scratch.wip_platform_block {
        scratch.wip_bind.set_for_platform(platform, 0);
    }

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::hex => {
                let (line, column) = inner.line_col();
                let value = parse_hex_literal(inner.as_str(), line, column)?;
                if let Some(platform) = scratch.wip_platform_block {
                    scratch.wip_bind.set_for_platform(platform, value);
                } else {
                    return Err(ParseError::PestError(
                        "must specify padding if not using platform expression".to_string(),
                    ));
                }
            }
            Rule::bind => {
                parse_bind_values(&inner, scratch)?;
            }
            _ => {}
        }
    }

    scratch.wip_field.inner = FieldInner::Pad(PadField {
        amount: std::mem::take(&mut scratch.wip_bind),
    });
    Ok(())
}

fn parse_member_expr(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    let mut platform = Platform::None;
    let mut name = String::new();
    let mut count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::platform => {
                if scratch.wip_platform_block.is_some() {
                    return Err(ParseError::PestError(
                        "cannot use platform inside platform expression".to_string(),
                    ));
                }
                platform = str_to_platform(inner.as_str());
            }
            Rule::type_content => {
                scratch.wip_type = parse_type_content(inner.as_str());
            }
            Rule::r#type => {
                scratch.wip_type = parse_type_from_pair(inner);
            }
            Rule::identifier => {
                name = inner.as_str().to_string();
            }
            Rule::number => {
                count = inner.as_str().parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    let final_platform = scratch.wip_platform_block.unwrap_or(platform);

    scratch.wip_field.inner = FieldInner::Member(MemberField {
        platform: final_platform,
        name,
        ty: std::mem::take(&mut scratch.wip_type),
        count,
    });
    Ok(())
}

fn parse_bind_expr(pair: pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    scratch.wip_mem_fn_proto = MemberFunctionProto::default();
    scratch.wip_attributes.links = scratch.wip_class.attributes.links;
    scratch.wip_attributes.missing = scratch.wip_class.attributes.missing;
    scratch.wip_fn_body.clear();
    scratch.wip_has_explicit_inline = false;
    scratch.wip_bind = PlatformNumber::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => {
                parse_attribute(&inner, scratch)?;
            }
            Rule::member_function_proto => {
                parse_member_function_proto(&inner, scratch)?;
            }
            Rule::bind => {
                parse_bind(&inner, scratch)?;
            }
            _ => {}
        }
    }

    scratch.wip_mem_fn_proto.attributes = std::mem::take(&mut scratch.wip_attributes);

    let has_inline = !scratch.wip_fn_body.is_empty() && !scratch.wip_has_explicit_inline;
    normalize_platform_number(&mut scratch.wip_bind, has_inline);

    scratch.wip_field.inner = FieldInner::FunctionBind(FunctionBindField {
        prototype: std::mem::take(&mut scratch.wip_mem_fn_proto),
        binds: std::mem::take(&mut scratch.wip_bind),
        inner: std::mem::take(&mut scratch.wip_fn_body),
    });
    Ok(())
}

fn parse_member_function_proto(
    pair: &pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<()> {
    let mut has_modifier = false;
    let mut is_destructor = false;

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::kw_protected => {
                scratch.wip_mem_fn_proto.access = AccessModifier::Protected;
            }
            Rule::kw_private => {
                scratch.wip_mem_fn_proto.access = AccessModifier::Private;
            }
            Rule::kw_static => {
                scratch.wip_mem_fn_proto.is_static = true;
                scratch.wip_mem_fn_proto.fn_type = FunctionType::Normal;
                has_modifier = true;
            }
            Rule::kw_virtual => {
                scratch.wip_mem_fn_proto.is_virtual = true;
                has_modifier = true;
            }
            Rule::kw_callback => {
                scratch.wip_mem_fn_proto.is_callback = true;
                has_modifier = true;
            }
            Rule::tilde => {
                is_destructor = true;
            }
            Rule::type_content => {
                scratch.wip_type = parse_type_content(inner.as_str());
                scratch.wip_mem_fn_proto.ret = scratch.wip_type.clone();
            }
            Rule::r#type => {
                scratch.wip_type = parse_type_from_pair(inner);
                scratch.wip_mem_fn_proto.ret = scratch.wip_type.clone();
            }
            Rule::identifier => {
                let name = inner.as_str();
                if is_destructor {
                    scratch.wip_mem_fn_proto.name = format!("~{}", name);
                    scratch.wip_mem_fn_proto.fn_type = FunctionType::Destructor;
                } else if !has_modifier && scratch.wip_mem_fn_proto.name.is_empty() {
                    scratch.wip_mem_fn_proto.name = name.to_string();
                    scratch.wip_mem_fn_proto.fn_type = FunctionType::Constructor;
                } else {
                    scratch.wip_mem_fn_proto.name = name.to_string();
                }
            }
            Rule::arg_list => {
                scratch.wip_mem_fn_proto.args = parse_arg_list(&inner)?;
            }
            Rule::kw_const => {
                scratch.wip_mem_fn_proto.is_const = true;
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_arg_list(pair: &pest::iterators::Pair<Rule>) -> Result<Vec<Arg>> {
    let mut args = Vec::new();
    let mut current_ty = Type::default();
    let mut pending_arg = false;

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::type_content => {
                if pending_arg && !current_ty.name.is_empty() {
                    args.push(Arg {
                        ty: std::mem::take(&mut current_ty),
                        name: String::new(),
                    });
                }
                current_ty = parse_type_content(inner.as_str());
                pending_arg = true;
            }
            Rule::r#type => {
                if pending_arg && !current_ty.name.is_empty() {
                    args.push(Arg {
                        ty: std::mem::take(&mut current_ty),
                        name: String::new(),
                    });
                }
                current_ty = parse_type_from_pair(inner);
                pending_arg = true;
            }
            Rule::identifier => {
                args.push(Arg {
                    ty: std::mem::take(&mut current_ty),
                    name: inner.as_str().to_string(),
                });
                pending_arg = false;
            }
            _ => {}
        }
    }

    if pending_arg && !current_ty.name.is_empty() {
        args.push(Arg {
            ty: current_ty,
            name: String::new(),
        });
    }

    for (i, arg) in args.iter_mut().enumerate() {
        if arg.name.is_empty() {
            arg.name = format!("p{}", i);
        }
    }

    Ok(args)
}

fn parse_bind(pair: &pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::bind_item => {
                parse_bind_item(&inner, scratch)?;
            }
            Rule::function_body => {
                scratch.wip_fn_body = inner.as_str().to_string();
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_bind_item(pair: &pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::bind_platform => {
                parse_bind_platform(&inner, scratch)?;
            }
            Rule::bind_inline => {
                scratch.wip_has_explicit_inline = true;
                for addr in [
                    &mut scratch.wip_bind.win,
                    &mut scratch.wip_bind.imac,
                    &mut scratch.wip_bind.m1,
                    &mut scratch.wip_bind.ios,
                    &mut scratch.wip_bind.android32,
                    &mut scratch.wip_bind.android64,
                ] {
                    if *addr == PlatformNumber::UNSPECIFIED {
                        *addr = PlatformNumber::INLINE;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_bind_platform(
    pair: &pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<()> {
    let mut platform = Platform::None;
    let mut value: isize = PlatformNumber::UNSPECIFIED;

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::platform => {
                platform = str_to_platform(inner.as_str());
            }
            Rule::kw_win
            | Rule::kw_mac
            | Rule::kw_ios
            | Rule::kw_android
            | Rule::kw_android32
            | Rule::kw_android64
            | Rule::kw_imac
            | Rule::kw_m1 => {
                platform = str_to_platform(inner.as_str());
            }
            Rule::hex => {
                let (line, column) = inner.line_col();
                value = parse_hex_literal(inner.as_str(), line, column)?;
            }
            Rule::kw_default => {
                value = PlatformNumber::DEFAULT;
            }
            Rule::kw_inline => {
                value = PlatformNumber::INLINE;
                scratch.wip_has_explicit_inline = true;
            }
            _ => {}
        }
    }

    scratch.wip_bind.set_for_platform(platform, value);
    Ok(())
}

fn parse_bind_values(pair: &pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    for inner in pair.clone().into_inner() {
        if inner.as_rule() == Rule::bind_platform {
            parse_bind_platform(&inner, scratch)?;
        }
    }
    Ok(())
}

fn parse_function(
    pair: pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<Function> {
    scratch.is_class = false;
    scratch.wip_fn_proto = FunctionProto::default();
    scratch.wip_fn_proto.attributes = std::mem::take(&mut scratch.wip_attributes);
    scratch.wip_fn_body.clear();
    scratch.wip_has_explicit_inline = false;
    scratch.wip_bind = PlatformNumber::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::function_proto => {
                parse_function_proto(&inner, scratch)?;
            }
            Rule::bind => {
                parse_bind(&inner, scratch)?;
            }
            _ => {}
        }
    }

    let has_inline = !scratch.wip_fn_body.is_empty() && !scratch.wip_has_explicit_inline;
    normalize_platform_number(&mut scratch.wip_bind, has_inline);

    Ok(Function {
        prototype: std::mem::take(&mut scratch.wip_fn_proto),
        binds: std::mem::take(&mut scratch.wip_bind),
        inner: std::mem::take(&mut scratch.wip_fn_body),
    })
}

fn parse_function_proto(
    pair: &pest::iterators::Pair<Rule>,
    scratch: &mut ScratchData,
) -> Result<()> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::type_content => {
                scratch.wip_type = parse_type_content(inner.as_str());
                scratch.wip_fn_proto.ret = scratch.wip_type.clone();
            }
            Rule::r#type => {
                scratch.wip_type = parse_type_from_pair(inner);
                scratch.wip_fn_proto.ret = scratch.wip_type.clone();
            }
            Rule::identifier => {
                scratch.wip_fn_proto.name = inner.as_str().to_string();
            }
            Rule::arg_list => {
                scratch.wip_fn_proto.args = parse_arg_list(&inner)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_attribute(pair: &pest::iterators::Pair<Rule>, scratch: &mut ScratchData) -> Result<()> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::docs_attribute => {
                let line = inner.as_str().trim_start_matches("///").trim();
                scratch.wip_attributes.docs.push_str(line);
                scratch.wip_attributes.docs.push('\n');
            }
            Rule::attribute_inner => {
                for attr in inner.into_inner() {
                    match attr.as_rule() {
                        Rule::depends_attribute => {
                            for dep in attr.into_inner() {
                                if dep.as_rule() == Rule::qualified {
                                    scratch
                                        .wip_attributes
                                        .depends
                                        .push(dep.as_str().to_string());
                                }
                            }
                        }
                        Rule::link_attribute => {
                            scratch.wip_attributes.links = Platform::None;
                            for p in attr.into_inner() {
                                if p.as_rule() == Rule::platform {
                                    scratch.wip_attributes.links |= str_to_platform(p.as_str());
                                }
                            }
                        }
                        Rule::missing_attribute => {
                            scratch.wip_attributes.missing = Platform::None;
                            for p in attr.into_inner() {
                                if p.as_rule() == Rule::platform {
                                    scratch.wip_attributes.missing |= str_to_platform(p.as_str());
                                }
                            }
                        }
                        Rule::since_attribute => {
                            for s in attr.into_inner() {
                                if s.as_rule() == Rule::string_literal {
                                    let lit = s.as_str();
                                    scratch.wip_attributes.since =
                                        lit[1..lit.len() - 1].to_string();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_type_from_pair(pair: pest::iterators::Pair<Rule>) -> Type {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::type_content {
            return parse_type_content(inner.as_str());
        }
    }
    Type::default()
}

fn parse_type_content(s: &str) -> Type {
    let s = s.trim();
    if s == "..." {
        return Type::new("...");
    }

    let mut result = Type::default();
    let mut name_parts: Vec<String> = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        let word: String = chars
            .by_ref()
            .take_while(|&c| {
                c.is_alphanumeric()
                    || c == '_'
                    || c == ':'
                    || c == '<'
                    || c == '>'
                    || c == ','
                    || c == ' '
            })
            .collect();

        match word.as_str() {
            "const" => name_parts.push("const".to_string()),
            "struct" => result.is_struct = true,
            "unsigned" => name_parts.push("unsigned".to_string()),
            "long" => name_parts.push("long".to_string()),
            w if !w.is_empty() => name_parts.push(w.to_string()),
            _ => {}
        }
    }

    let ptr_chars: String = s.chars().filter(|&c| c == '*' || c == '&').collect();

    let mut name = name_parts.join(" ");
    if !ptr_chars.is_empty() {
        name.push(' ');
        for c in ptr_chars.chars() {
            if c == '*' {
                name.push('*');
            } else if c == '&' {
                name.push('&');
            }
        }
    }

    result.name = name.trim().to_string();
    result
}

fn parse_hex_literal(s: &str, line: usize, column: usize) -> Result<isize> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    isize::from_str_radix(s, 16).map_err(|_| ParseError::InvalidHexLiteral {
        value: s.to_string(),
        line,
        column,
    })
}

fn str_to_platform(s: &str) -> Platform {
    match s {
        "win" | "windows" => Platform::Windows,
        "mac" => Platform::Mac,
        "imac" => Platform::MacIntel,
        "m1" => Platform::MacArm,
        "ios" => Platform::IOS,
        "android" => Platform::Android,
        "android32" => Platform::Android32,
        "android64" => Platform::Android64,
        _ => Platform::None,
    }
}

fn normalize_platform_number(bind: &mut PlatformNumber, has_inline: bool) {
    let addrs = [
        &mut bind.win,
        &mut bind.imac,
        &mut bind.m1,
        &mut bind.ios,
        &mut bind.android32,
        &mut bind.android64,
    ];

    for addr in addrs {
        if *addr == PlatformNumber::UNSPECIFIED && has_inline {
            *addr = PlatformNumber::INLINE;
        } else if *addr == PlatformNumber::DEFAULT {
            *addr = PlatformNumber::UNSPECIFIED;
        }
    }
}

fn post_process(root: &mut Root) {
    for class in &mut root.classes {
        if class.attributes.links == Platform::None {
            class.attributes.links = Platform::All;
        }
        if class.attributes.missing == Platform::None {
            class.attributes.missing = Platform::None;
        }
    }
}
