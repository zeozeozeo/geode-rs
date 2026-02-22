use broma_rs::ast::{AccessModifier, FunctionType, Platform};
use broma_rs::parse_file;
use std::path::Path;

#[test]
fn test_parse_class_bro() {
    let result = parse_file(Path::new("tests/class.bro")).expect("failed to parse class.bro");
    assert!(!result.classes.is_empty());
}

#[test]
fn test_parse_free_bro() {
    let result = parse_file(Path::new("tests/free.bro")).expect("failed to parse free.bro");
    assert!(!result.functions.is_empty());
}

#[test]
fn test_parse_cocos2d() {
    let result = parse_file(Path::new("tests/Cocos2d.bro")).expect("failed to parse Cocos2d.bro");
    assert!(!result.classes.is_empty());
}

#[test]
fn test_parse_geometrydash() {
    let result =
        parse_file(Path::new("tests/GeometryDash.bro")).expect("failed to parse GeometryDash.bro");
    assert!(!result.classes.is_empty());
}

#[test]
fn test_parse_entry() {
    let result = parse_file(Path::new("tests/Entry.bro")).expect("failed to parse Entry.bro");
    assert_eq!(result.headers.len(), 5);
    let names: Vec<&str> = result.headers.iter().map(|h| h.name.as_str()).collect();
    assert!(names.contains(&"Cocos2d.bro"), "headers: {:?}", names);
}

#[test]
fn test_parse_extras() {
    let result = parse_file(Path::new("tests/Extras.bro")).expect("failed to parse Extras.bro");
    assert!(!result.classes.is_empty());
}

#[test]
fn test_parse_fmod() {
    let result = parse_file(Path::new("tests/FMOD.bro")).expect("failed to parse FMOD.bro");
    assert!(!result.classes.is_empty());
}

#[test]
fn test_parse_kazmath() {
    let result = parse_file(Path::new("tests/Kazmath.bro")).expect("failed to parse Kazmath.bro");
    assert!(!result.functions.is_empty() || !result.classes.is_empty());
}

#[test]
fn test_class_bro_contents() {
    let result = parse_file(Path::new("tests/class.bro")).expect("failed to parse class.bro");
    let test_class = result.find_class("Test").expect("Test class not found");

    assert_eq!(test_class.name, "Test");
    assert!(test_class.attributes.links.contains(Platform::Android));

    let member_field = test_class
        .find_field("member")
        .expect("member field not found");
    let member = member_field
        .as_function_bind()
        .expect("member should be a function bind");
    assert_eq!(member.prototype.name, "member");
    assert_eq!(member.prototype.ret.name, "int");
    assert_eq!(member.prototype.args.len(), 1);
    assert_eq!(member.prototype.args[0].ty.name, "std::string");
    assert_eq!(member.prototype.args[0].name, "str");

    let member2_field = test_class
        .find_field("member2")
        .expect("member2 field not found");
    let member2 = member2_field
        .as_function_bind()
        .expect("member2 should be a function bind");
    assert_eq!(member2.prototype.access, AccessModifier::Protected);

    let bound_explicit_inline = test_class
        .find_field("bound_explicit_inline")
        .expect("bound_explicit_inline not found");
    let fn_bind = bound_explicit_inline
        .as_function_bind()
        .expect("should be function bind");
    assert_eq!(fn_bind.binds.win, 0x433);
    assert_eq!(fn_bind.binds.ios, -2);

    let bound_implicit_inline = test_class
        .find_field("bound_implicit_inline")
        .expect("bound_implicit_inline not found");
    let fn_bind2 = bound_implicit_inline
        .as_function_bind()
        .expect("should be function bind");
    assert_eq!(fn_bind2.binds.ios, 0x5467);

    let thing = test_class.find_field("thing").expect("thing not found");
    let thing_bind = thing.as_function_bind().expect("should be function bind");
    assert_eq!(thing_bind.binds.win, 0x5);
    assert_eq!(thing_bind.binds.imac, 0x8);
    assert_eq!(thing_bind.binds.m1, 0x4);
    assert_eq!(thing_bind.binds.ios, -1);

    let m_test_begin = test_class
        .find_field("m_testBegin")
        .expect("m_testBegin not found");
    let member_field = m_test_begin.as_member().expect("should be member field");
    assert_eq!(member_field.name, "m_testBegin");
    assert_eq!(member_field.ty.name, "int");

    let m_test_main = test_class
        .find_field("m_testMain")
        .expect("m_testMain not found");
    let member_main = m_test_main.as_member().expect("should be member field");
    assert_eq!(member_main.name, "m_testMain");
    assert_eq!(member_main.ty.name, "char");

    let new_feature = test_class
        .find_field("new_feature")
        .expect("new_feature not found");
    let _new_feature_bind = new_feature
        .as_function_bind()
        .expect("should be function bind");
}

#[test]
fn test_free_bro_contents() {
    let result = parse_file(Path::new("tests/free.bro")).expect("failed to parse free.bro");

    assert_eq!(result.functions.len(), 3);

    let say_hello = result
        .functions
        .iter()
        .find(|f| f.prototype.name == "say_hello")
        .expect("say_hello not found");
    assert_eq!(say_hello.prototype.ret.name, "void");
    assert_eq!(say_hello.prototype.args.len(), 3);
    assert_eq!(say_hello.binds.win, 0x0);
    assert_eq!(say_hello.binds.imac, 0x24);
    assert_eq!(say_hello.binds.m1, 0x554);
    assert_eq!(say_hello.binds.ios, 0x343);

    let i_hate_mat = result
        .functions
        .iter()
        .find(|f| f.prototype.name == "i_hate_mat")
        .expect("i_hate_mat not found");
    assert_eq!(i_hate_mat.prototype.args.len(), 3);
    assert_eq!(i_hate_mat.prototype.args[0].ty.name, "std::string");
    assert_eq!(i_hate_mat.prototype.args[1].ty.name, "std::vector<int>");
    assert_eq!(i_hate_mat.binds.imac, 0x33);
    assert_eq!(i_hate_mat.binds.m1, 0x33);

    let uhhhhhh = result
        .functions
        .iter()
        .find(|f| f.prototype.name == "uhhhhhh")
        .expect("uhhhhhh not found");
    assert_eq!(uhhhhhh.binds.win, 0x3434);
}

#[test]
fn test_cocos2d_contents() {
    let result = parse_file(Path::new("tests/Cocos2d.bro")).expect("failed to parse Cocos2d.bro");

    let cccontent = result
        .find_class("CCContentManager")
        .expect("CCContentManager not found");
    assert!(
        cccontent
            .superclasses
            .iter()
            .any(|s| s == "cocos2d::CCObject")
    );
    assert!(cccontent.attributes.links.contains(Platform::Windows));
    assert!(cccontent.attributes.links.contains(Platform::Android));

    let shared_manager = cccontent
        .find_field("sharedManager")
        .expect("sharedManager not found");
    let sm_bind = shared_manager
        .as_function_bind()
        .expect("should be function bind");
    assert!(sm_bind.prototype.is_static);
    assert!(sm_bind.prototype.ret.name.contains("CCContentManager"));

    let cocos2d_class = result
        .find_class("cocos2d")
        .expect("cocos2d class not found");
    let cc_log = cocos2d_class.find_field("CCLog").expect("CCLog not found");
    let cc_log_bind = cc_log.as_function_bind().expect("should be function bind");
    assert!(cc_log_bind.prototype.is_static);
    assert!(!cc_log_bind.prototype.args.is_empty());

    let cc_action = result
        .find_class("cocos2d::CCAction")
        .expect("CCAction not found");

    let destructor = cc_action
        .fields
        .iter()
        .filter_map(|f| f.as_function_bind())
        .find(|b| b.prototype.fn_type == FunctionType::Destructor)
        .expect("destructor not found");
    assert_eq!(destructor.prototype.fn_type, FunctionType::Destructor);

    let cc_action_inst = result
        .find_class("cocos2d::CCActionInstant")
        .expect("CCActionInstant not found");
    let ctor = cc_action_inst
        .find_field("CCActionInstant")
        .expect("constructor not found");
    let ctor_bind = ctor.as_function_bind().expect("should be function bind");
    assert_eq!(ctor_bind.prototype.fn_type, FunctionType::Constructor);

    let m_tag = cc_action.find_field("m_nTag").expect("m_nTag not found");
    let tag_member = m_tag.as_member().expect("should be member");
    assert_eq!(tag_member.ty.name, "int");
}

#[test]
fn test_geometrydash_contents() {
    let result =
        parse_file(Path::new("tests/GeometryDash.bro")).expect("failed to parse GeometryDash.bro");

    let account_help = result
        .find_class("AccountHelpLayer")
        .expect("AccountHelpLayer not found");
    assert!(
        account_help
            .superclasses
            .iter()
            .any(|s| s == "GJDropDownLayer")
    );

    let create_fn = account_help.find_field("create").expect("create not found");
    let create_bind = create_fn
        .as_function_bind()
        .expect("should be function bind");
    assert!(create_bind.prototype.is_static);

    let dest = account_help
        .fields
        .iter()
        .filter_map(|f| f.as_function_bind())
        .find(|b| b.prototype.fn_type == FunctionType::Destructor)
        .expect("destructor not found");
    assert_eq!(dest.prototype.fn_type, FunctionType::Destructor);

    let m_label = account_help
        .find_field("m_loginStatusLabel")
        .expect("m_loginStatusLabel not found");
    let label_member = m_label.as_member().expect("should be member");
    assert!(label_member.ty.name.contains("CCLabelBMFont"));
}

#[test]
fn test_extras_contents() {
    let result = parse_file(Path::new("tests/Extras.bro")).expect("failed to parse Extras.bro");

    let fmod_sound = result.find_class("FMODSound").expect("FMODSound not found");
    let sound_member = fmod_sound.find_field("m_sound").expect("m_sound not found");
    let member = sound_member.as_member().expect("should be member");
    assert!(member.ty.name.contains("FMOD::Sound"));

    let mut found_depends = false;
    for class in &result.classes {
        if !class.attributes.depends.is_empty() {
            found_depends = true;
            break;
        }
    }
    assert!(
        found_depends,
        "at least one class should have depends attribute"
    );
}

#[test]
fn test_platform_number_inline() {
    let result = parse_file(Path::new("tests/class.bro")).expect("failed to parse class.bro");
    let test_class = result.find_class("Test").expect("Test class not found");

    let normal_inline = test_class
        .find_field("normal_inline")
        .expect("normal_inline not found");
    let bind = normal_inline
        .as_function_bind()
        .expect("should be function bind");
    assert_eq!(bind.binds.win, -2);
    assert_eq!(bind.binds.imac, -2);
    assert_eq!(bind.binds.m1, -2);
    assert!(bind.binds.ios == -2 || bind.binds.ios == -1);
    assert!(!bind.inner.is_empty());
}
