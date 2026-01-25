use std::{collections::HashSet, sync::Arc};

use gtk4::{
    TextBuffer,
    prelude::{TextBufferExt, TextBufferExtManual},
};
use tree_sitter::Parser;
use unicode_segmentation::UnicodeSegmentation;

use crate::tree_sitter_qat;

pub fn setup_highlighting_for_qat(buffer: Arc<TextBuffer>) {
    let tag_keyword = buffer
        .create_tag(Some("keyword"), &[("foreground", &"#ff88cd")])
        .expect("Could not create tag for keywords");
    let tag_function = buffer
        .create_tag(
            Some("function"),
            &[("foreground", &"#69a5ff"), ("weight", &700)],
        )
        .expect("Could not create tag for function names");
    let tag_type = buffer
        .create_tag(
            Some("type"),
            &[("foreground", &"#fbd37d"), ("weight", &700)],
        )
        .expect("Could not create tag for type");
    let tag_type_builtin = buffer
        .create_tag(
            Some("type_builtin"),
            &[("foreground", &"#b29bff"), ("weight", &700)],
        )
        .expect("Could not create tag for builtin types");
    let tag_constant = buffer
        .create_tag(Some("constant"), &[("foreground", &"#ffb293")])
        .expect("Could not create tag for constants");
    let tag_string = buffer
        .create_tag(Some("string"), &[("foreground", &"#a5ff8e")])
        .expect("Could not create tag for strings");
    let tag_escape_string = buffer
        .create_tag(
            Some("escape_string"),
            &[("foreground", &"#c5ffff"), ("weight", &700)],
        )
        .expect("Could not create tag for escape strings");
    let tag_dead = buffer
        .create_tag(Some("dead"), &[("foreground", &"#535353")])
        .expect("Could not create tag for dead code");
    let tag_field = buffer
        .create_tag(Some("field"), &[("foreground", &"#ff7272")])
        .expect("Could not create tag for fields");
    let tag_important = buffer
        .create_tag(
            Some("important"),
            &[("foreground", &"#ffffff"), ("weight", &700)],
        )
        .expect("Could not create tag for important entities");
    let tag_symbols = buffer
        .create_tag(Some("symbols"), &[("foreground", &"#c3c3c3")])
        .expect("Could not create tag for symbols");
    let keyword_list: HashSet<&str> = [
        "pub", "give", "loop", "struct", "mix", "toggle", "choice", "region", "heap", "is", "in",
        "own", "let", "meta", "define", "if", "where", "use", "copy", "move", "swap", "pre", "say",
        "not", "or", "and", "do", "skill", "type", "for", "else", "match", "var", "variadic",
        "assembly", "from", "to", "flag", "opaque", "end", "operator", "spawn", "ignore", "_",
        "default", "as", "volatile", "ok", "try",
    ]
    .iter()
    .cloned()
    .collect();
    let builtin_types: HashSet<&str> = [
        "atomic",
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "u1",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "f32",
        "f64",
        "f80",
        "f128",
        "f128ppc",
        "fbrain",
        "int",
        "uint",
        "byteptr",
        "float",
        "double",
        "longdouble",
        "usize",
        "isize",
        "self",
        "bool",
        "byte",
        "char",
        "uchar",
        "poly",
        "maybe",
        "result",
        "error",
        "ref",
        "ptr",
        "multi",
        "text",
        "slice",
        "future",
        "integer",
        "vec",
    ]
    .iter()
    .cloned()
    .collect();
    let constant_list: HashSet<&str> = ["none", "null"].iter().cloned().collect();
    let change_fn = move |buf: &'_ TextBuffer| {
        let content = buf.text(&buf.start_iter(), &buf.end_iter(), true);
        let mut grapheme_indices: Vec<i32> = vec![0; content.len()];
        for (grapheme_index, (byte_index, grapheme)) in content.grapheme_indices(true).enumerate() {
            for offset in 0..grapheme.len() {
                grapheme_indices[byte_index + offset] = grapheme_index as i32;
            }
        }
        let keyword_list = &keyword_list;
        let tag_keyword = &tag_keyword;
        let tag_type = &tag_type;
        let tag_type_builtin = &tag_type_builtin;
        let tag_function = &tag_function;
        let tag_dead = &tag_dead;
        let tag_constant = &tag_constant;
        let tag_string = &tag_string;
        let tag_field = &tag_field;
        let tag_important = &tag_important;
        let tag_escape_string = &tag_escape_string;
        let tag_symbols = &tag_symbols;
        let language = unsafe { tree_sitter_qat() };
        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .expect("Could not set language");
        let (start, end) = buf.bounds();
        let changed_content = buf.text(&start, &end, false);
        let important_symbols: HashSet<&str> = ["'", "''", "{", "}", ":=", ".", "->", "<-"]
            .iter()
            .cloned()
            .collect();
        let passive_symbols: HashSet<&str> = ["[", "]", ":[", "::", ";", ",", ":"]
            .iter()
            .cloned()
            .collect();
        if let Some(tree) = parser.parse(changed_content, None) {
            let mut cursor = tree.walk();
            buf.remove_all_tags(&buf.start_iter(), &buf.end_iter());
            'outer: loop {
                let node = cursor.node();
                if !node.is_named() {
                    if keyword_list.contains(&node.kind()) {
                        let range = node.byte_range();
                        let start = buf.iter_at_offset(grapheme_indices[range.start]);
                        let end =
                            buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                        buf.apply_tag(tag_keyword, &start, &end);
                    } else if builtin_types.contains(&node.kind()) {
                        let range = node.byte_range();
                        let start = buf.iter_at_offset(grapheme_indices[range.start]);
                        let end =
                            buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                        buf.apply_tag(tag_type_builtin, &start, &end);
                    } else if constant_list.contains(&node.kind()) {
                        let range = node.byte_range();
                        let start = buf.iter_at_offset(grapheme_indices[range.start]);
                        let end =
                            buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                        buf.apply_tag(tag_constant, &start, &end);
                    } else if important_symbols.contains(node.kind()) {
                        let range = node.byte_range();
                        let start = buf.iter_at_offset(grapheme_indices[range.start]);
                        let end =
                            buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                        buf.apply_tag(tag_important, &start, &end);
                    } else if passive_symbols.contains(node.kind()) {
                        let range = node.byte_range();
                        let start = buf.iter_at_offset(grapheme_indices[range.start]);
                        let end =
                            buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                        buf.apply_tag(tag_symbols, &start, &end);
                    }
                } else {
                    let range = node.byte_range();
                    let start = buf.iter_at_offset(grapheme_indices[range.start]);
                    let end =
                        buf.iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                    match node.kind() {
                        "self_instance" => {
                            let range = node.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            buf.apply_tag(tag_important, &start, &end);
                        }
                        "comment_line" | "comment_multi" => {
                            buf.apply_tag(tag_dead, &start, &end);
                        }
                        "literal_string" | "multiline_string" => {
                            buf.apply_tag(tag_string, &start, &end);
                        }
                        "escape_sequence" => {
                            buf.apply_tag(tag_escape_string, &start, &end);
                        }
                        "constants" | "literal_integer" => {
                            buf.apply_tag(tag_constant, &start, &end);
                        }
                        "type" => {
                            let child = node.child(0).expect("Could not find child node in type");
                            if child.kind() == "entity" {
                                let name_field = child.child_by_field_name("name").expect(
                                    ("Could not get name field in ".to_owned() + &node.kind())
                                        .as_str(),
                                );
                                let range = name_field.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                if builtin_types.contains(
                                    &content[node.byte_range().start..node.byte_range().end],
                                ) {
                                    buf.apply_tag(tag_type_builtin, &start, &end);
                                } else if node.parent().is_none() {
                                    buf.apply_tag(tag_type, &start, &end);
                                }
                            }
                        }
                        "type_subtype" => {
                            let mut tag_value = tag_type;
                            if let Some(parent1) = node.parent()
                                && parent1.kind() == "type_without_entity"
                                && node.prev_sibling().is_none()
                            {
                                if let Some(parent2) = parent1.parent()
                                    && parent1.prev_sibling().is_none()
                                {
                                    if parent2.kind() == "function_call" {
                                        tag_value = tag_function;
                                    } else if parent2.kind() == "type" {
                                        if let Some(parent3) = parent2.parent()
                                            && parent3.kind() == "type_generic"
                                            && parent2.prev_sibling().is_none()
                                        {
                                            if let Some(parent4) = parent3.parent()
                                                && parent4.kind() == "type_without_entity"
                                                && parent3.prev_sibling().is_none()
                                            {
                                                if let Some(parent5) = parent4.parent()
                                                    && parent5.kind() == "function_call"
                                                    && parent4.prev_sibling().is_none()
                                                {
                                                    tag_value = tag_function;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            let mut cursor = tree.walk();
                            if let Some(id) = node.children(&mut cursor).last() {
                                let range = id.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_value, &start, &end);
                            }
                        }
                        "type_primitive" | "type_signed_integer" | "type_unsigned_integer" => {
                            buf.apply_tag(tag_type_builtin, &start, &end);
                        }
                        "type_generic" => {
                            let child = node.child(0).expect("Cannot find child in type_generic");
                            let mut tag_value = tag_type;
                            if let Some(type_without_entity) = node.parent()
                                && node.prev_sibling().is_none()
                                && type_without_entity.kind() == "type_without_entity"
                            {
                                if let Some(parent) = type_without_entity.parent()
                                    && type_without_entity.prev_sibling().is_none()
                                    && parent.kind() == "function_call"
                                {
                                    tag_value = tag_function;
                                }
                            }
                            if let Some(second_child) = child.child(0) {
                                if second_child.kind() == "entity" {
                                    let mut cursor = tree.walk();
                                    let last = second_child
                                        .children(&mut cursor)
                                        .last()
                                        .expect("Could not get last child in entity");
                                    let range = last.byte_range();
                                    let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                    let end = buf.iter_at_offset(
                                        grapheme_indices[range.end.min(content.len() - 1)],
                                    );
                                    buf.apply_tag(tag_value, &start, &end);
                                }
                            }
                        }
                        "function_definition" | "prerun_function_definition" | "method" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            buf.apply_tag(tag_function, &start, &end);
                        }
                        "struct_definition" | "mix_definition" | "toggle_definition"
                        | "choice_definition" | "flag_definition" | "type_definition"
                        | "skill_definition" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            buf.apply_tag(tag_type, &start, &end);
                        }
                        "struct_field"
                        | "flag_field"
                        | "statement_declaration"
                        | "mix_field"
                        | "choice_field_name"
                        | "toggle_field"
                        | "function_parameter_single" => {
                            if let Some(name_field) = node.child_by_field_name("name") {
                                let range = name_field.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_field, &start, &end);
                            }
                        }
                        "method_arg_single" => {
                            if let Some(name_field) = node.child_by_field_name("member") {
                                let range = name_field.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_field, &start, &end);
                            }
                        }
                        "generic_parameter_single" => {
                            if let Some(name_field) = node.child_by_field_name("type_parameter") {
                                let range = name_field.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_type, &start, &end);
                            }
                            if let Some(name_field) = node.child_by_field_name("prerun_parameter") {
                                let range = name_field.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_constant, &start, &end);
                            }
                        }
                        "flag_is_variant" | "flag_initialiser" => {
                            let mut cursor = tree.walk();
                            for name in node.children_by_field_name("name", &mut cursor) {
                                let range = name.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                buf.apply_tag(tag_field, &start, &end);
                            }
                        }
                        "entity" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            if builtin_types
                                .contains(&content[node.byte_range().start..node.byte_range().end])
                            {
                                buf.apply_tag(tag_type_builtin, &start, &end);
                            } else if keyword_list
                                .contains(&content[node.byte_range().start..node.byte_range().end])
                            {
                                buf.apply_tag(tag_keyword, &start, &end);
                            } else if constant_list
                                .contains(&content[node.byte_range().start..node.byte_range().end])
                            {
                                buf.apply_tag(tag_constant, &start, &end);
                            } else {
                                if let Some(parent) = node.parent() {
                                    match parent.kind() {
                                        "function_call" | "type" | "type_generic" => {}
                                        _ => {
                                            if node.child_count() == 1 {
                                                buf.apply_tag(tag_field, &start, &end);
                                            }
                                        }
                                    }
                                } else if node.child_count() == 1 {
                                    buf.apply_tag(tag_field, &start, &end);
                                }
                            }
                        }
                        "function_call" => {
                            let child = node
                                .child(0)
                                .expect("Could not get child node in function_call");
                            if child.kind() == "entity" {
                                let value =
                                    &content[child.byte_range().start..child.byte_range().end];
                                let name = child
                                    .child_by_field_name("name")
                                    .expect("Could not get name field in entity in function_call");
                                let range = name.byte_range();
                                let start = buf.iter_at_offset(grapheme_indices[range.start]);
                                let end = buf.iter_at_offset(
                                    grapheme_indices[range.end.min(content.len() - 1)],
                                );
                                if builtin_types.contains(value) {
                                    buf.apply_tag(tag_type_builtin, &start, &end);
                                } else if keyword_list.contains(value) {
                                    buf.apply_tag(tag_keyword, &start, &end);
                                } else {
                                    buf.apply_tag(tag_function, &start, &end);
                                }
                            } else {
                                println!(
                                    "Function call {} child is {}",
                                    &content[child.byte_range().start..child.byte_range().end],
                                    child.kind()
                                );
                            }
                        }
                        "constructor_call" => {
                            let child = node
                                .child(0)
                                .expect("Could not get child of constructor call");
                            println!(
                                "Child of constructor call {} is {}",
                                &content[node.byte_range().start..node.byte_range().end],
                                child.kind()
                            );
                        }
                        "mix_initialiser" | "choice_initialiser" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            buf.apply_tag(
                                if node.kind() == "mix_initialiser" {
                                    tag_type
                                } else {
                                    tag_field
                                },
                                &start,
                                &end,
                            );
                        }
                        "heap_get" | "heap_put" | "heap_grow" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            buf.apply_tag(tag_function, &start, &end);
                        }
                        "member_access" => {
                            let name_field = node.child_by_field_name("name").expect(
                                ("Could not get name field in ".to_owned() + &node.kind()).as_str(),
                            );
                            let range = name_field.byte_range();
                            let start = buf.iter_at_offset(grapheme_indices[range.start]);
                            let end = buf
                                .iter_at_offset(grapheme_indices[range.end.min(content.len() - 1)]);
                            if let Some(parent) = node.parent() {
                                if parent.kind() == "function_call" {
                                    buf.apply_tag(tag_function, &start, &end);
                                } else {
                                    buf.apply_tag(tag_field, &start, &end);
                                }
                            } else {
                                buf.apply_tag(tag_field, &start, &end);
                            }
                        }
                        _ => {}
                    }
                }
                if cursor.goto_first_child() {
                    continue;
                }
                if cursor.goto_next_sibling() {
                    continue;
                }
                loop {
                    if cursor.goto_parent() {
                        if cursor.goto_next_sibling() {
                            continue 'outer;
                        } else {
                            continue;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    };
    change_fn(&buffer);
    buffer.connect_changed(change_fn);
}
