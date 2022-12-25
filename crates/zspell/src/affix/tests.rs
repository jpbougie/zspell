//! Affix tests
use util::TestCollection;

use super::*;
use crate::affix::types::AffixRule;

#[test]
fn test_check_condition() {
    let mut ard = AffixRule {
        strip: None,
        affix: String::new(),
        condition: None,
        morph_info: Vec::new(),
    };
    let mut kind = RuleType::Suffix;
    ard.set_re_pattern("[^aeiou]y", kind);

    // General tests, including with pattern in the middle
    assert!(ard.check_condition("xxxy"));
    assert!(!ard.check_condition("xxxay"));
    assert!(!ard.check_condition("xxxyxx"));

    // Test with prefix
    kind = RuleType::Prefix;
    ard.set_re_pattern("y[^aeiou]", kind);
    assert!(ard.check_condition("yxxx"));
    assert!(!ard.check_condition("yaxxx"));
    assert!(!ard.check_condition("xxxyxxx"));

    // Test other real rules
    kind = RuleType::Suffix;
    ard.set_re_pattern("[sxzh]", kind);
    assert!(ard.check_condition("access"));
    assert!(ard.check_condition("abyss"));
    assert!(!ard.check_condition("accomplishment"));
    assert!(ard.check_condition("mmms"));
    assert!(!ard.check_condition("mmsmm"));

    // Check with default condition
    ard.set_re_pattern(".", kind);
    assert!(ard.check_condition("xxx"));
}

#[test]
fn test_apply_pattern() {
    let mut kind = RuleType::Suffix;
    let mut rule = AffixRule::new(kind, "zzz", Some("y"), None, Vec::new()).unwrap();

    rule.set_re_pattern("[^aeiou]y", kind);
    assert_eq!(rule.apply_pattern("xxxy", kind), Some("xxxzzz".to_owned()));

    kind = RuleType::Prefix;
    rule.set_re_pattern("y[^aeiou]", kind);
    assert_eq!(rule.apply_pattern("yxxx", kind), Some("zzzxxx".to_owned()));

    kind = RuleType::Suffix;
    rule.set_re_pattern(".", kind);
    assert_eq!(rule.apply_pattern("xxx", kind), Some("xxxzzz".to_owned()));
}

#[test]
fn test_rule_group_apply_pattern() {
    let kind = RuleType::Suffix;
    let group = RuleGroup {
        flag: "A".to_owned(),
        kind: RuleType::Suffix,
        can_combine: true,
        rules: vec![
            AffixRule::new(kind, "iness", Some("y"), Some("[^aeiou]y"), Vec::new()).unwrap(),
            AffixRule::new(kind, "ness", None, Some("[aeiou]y"), Vec::new()).unwrap(),
            AffixRule::new(kind, "ness", None, Some("[^y]"), Vec::new()).unwrap(),
        ],
    };

    assert_eq!(group.apply_pattern("blurry").unwrap(), "blurriness");
    assert_eq!(group.apply_pattern("coy").unwrap(), "coyness");
    assert_eq!(group.apply_pattern("acute").unwrap(), "acuteness");
}

#[test]
fn affix_create_words() {
    // Does not yet check the rules component
    fn map_tuples<'a>(
        tup: &'a Vec<(String, &'a AffixRule, Option<&'a AffixRule>)>,
    ) -> Vec<&'a str> {
        tup.iter().map(|t| t.0.as_str()).collect()
    }

    let content = TestCollection::load("1_pfxsfx.test").afx_str;
    let afx = Config::load_from_str(content.as_str()).unwrap();

    assert_eq!(
        map_tuples(&afx.create_affixed_words("xxx", &["A"])),
        vec!["aaxxx"]
    );
    assert_eq!(
        map_tuples(&afx.create_affixed_words("xxx", &["B"])),
        vec!["xxxcc"]
    );
    assert_eq!(
        map_tuples(&afx.create_affixed_words("xxx", &["A", "B"])),
        vec!["aaxxx", "xxxcc", "aaxxxcc",]
    );
}
