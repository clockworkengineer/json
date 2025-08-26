use crate::json_lib::nodes::node::*;
use crate::json_lib::io::traits::IDestination;

fn escape_xml_string(s: &str, destination: &mut dyn IDestination) {
    for c in s.chars() {
        match c {
            '<' => destination.add_bytes("&lt;"),
            '>' => destination.add_bytes("&gt;"),
            '&' => destination.add_bytes("&amp;"),
            '"' => destination.add_bytes("&quot;"),
            '\'' => destination.add_bytes("&apos;"),
            c => destination.add_bytes(&c.to_string()),
        }
    }
}

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::None => destination.add_bytes("<null/>"),
        Node::Boolean(value) => {
            destination.add_bytes("<boolean>");
            destination.add_bytes(if *value { "true" } else { "false" });
            destination.add_bytes("</boolean>");
        }
        Node::Number(value) => {
            destination.add_bytes("<number>");
            match value {
                Number::Integer(n) => destination.add_bytes(&n.to_string()),
                Number::UInteger(n) => destination.add_bytes(&n.to_string()),
                Number::Float(f) => destination.add_bytes(&f.to_string()),
                Number::Byte(b) => destination.add_bytes(&b.to_string()),
                Number::Int32(i) => destination.add_bytes(&i.to_string()),
                Number::UInt32(u) => destination.add_bytes(&u.to_string()),
                #[allow(unreachable_patterns)]
                _ => destination.add_bytes(&format!("{:?}", value)),
            }
            destination.add_bytes("</number>");
        }
        Node::Str(value) => {
            destination.add_bytes("<string>");
            escape_xml_string(value, destination);
            destination.add_bytes("</string>");
        }
        Node::Array(items) => {
            destination.add_bytes("<array>");
            for item in items {
                destination.add_bytes("<item>");
                stringify(item, destination);
                destination.add_bytes("</item>");
            }
            destination.add_bytes("</array>");
        }
        Node::Object(entries) => {
            destination.add_bytes("<object>");
            for (key, value) in entries {
                destination.add_bytes("<entry>");
                destination.add_bytes("<key>");
                escape_xml_string(key, destination);
                destination.add_bytes("</key>");
                destination.add_bytes("<value>");
                stringify(value, destination);
                destination.add_bytes("</value>");
                destination.add_bytes("</entry>");
            }
            destination.add_bytes("</object>");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::json_lib::io::destinations::buffer::Buffer;

    #[test]
    fn test_stringify_null() {
        let mut dest = Buffer::new();
        stringify(&Node::None, &mut dest);
        assert_eq!(dest.to_string(), "<null/>");
    }

    #[test]
    fn test_stringify_boolean() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(true), &mut dest);
        assert_eq!(dest.to_string(), "<boolean>true</boolean>");
    }

    #[test]
    fn test_stringify_boolean_false() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(false), &mut dest);
        assert_eq!(dest.to_string(), "<boolean>false</boolean>");
    }

    #[test]
    fn test_stringify_number() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Float(42.5)), &mut dest);
        assert_eq!(dest.to_string(), "<number>42.5</number>");
    }

    #[test]
    fn test_stringify_all_numbers() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Integer(-42)), &mut dest);
        assert_eq!(dest.to_string(), "<number>-42</number>");

        dest = Buffer::new();
        stringify(&Node::Number(Number::UInteger(42)), &mut dest);
        assert_eq!(dest.to_string(), "<number>42</number>");

        dest = Buffer::new();
        stringify(&Node::Number(Number::Byte(255)), &mut dest);
        assert_eq!(dest.to_string(), "<number>255</number>");

        dest = Buffer::new();
        stringify(&Node::Number(Number::Int32(-2147483648)), &mut dest);
        assert_eq!(dest.to_string(), "<number>-2147483648</number>");

        dest = Buffer::new();
        stringify(&Node::Number(Number::UInt32(4294967295)), &mut dest);
        assert_eq!(dest.to_string(), "<number>4294967295</number>");
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("Hello & World".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "<string>Hello &amp; World</string>");
    }

    #[test]
    fn test_stringify_special_chars() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("<>&\"'".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "<string>&lt;&gt;&amp;&quot;&apos;</string>");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Number(Number::Integer(1)),
            Node::Str("test".to_string()),
        ]), &mut dest);
        assert_eq!(dest.to_string(), "<array><item><number>1</number></item><item><string>test</string></item></array>");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Object(map), &mut dest);
        assert_eq!(dest.to_string(), "<object><entry><key>key</key><value><string>value</string></value></entry></object>");
    }

    #[test]
    fn test_stringify_complex_nested() {
        let mut dest = Buffer::new();
        let mut inner_map = HashMap::new();
        inner_map.insert("inner".to_string(), Node::Array(vec![
            Node::Boolean(true),
            Node::None,
            Node::Number(Number::Float(42.5))
        ]));
        let mut outer_map = HashMap::new();
        outer_map.insert("outer<".to_string(), Node::Object(inner_map));
        stringify(&Node::Object(outer_map), &mut dest);
        assert_eq!(dest.to_string(), "<object><entry><key>outer&lt;</key><value><object><entry><key>inner</key><value><array><item><boolean>true</boolean></item><item><null/></item><item><number>42.5</number></item></array></value></entry></object></value></entry></object>");
    }
}