use matching_brackets::brackets_are_balanced;
use matching_brackets::get_match;
use matching_brackets::is_bracket;

#[test]
fn is_a_bracket() {
    assert!(is_bracket('['));
}

#[test]
fn not_a_bracket() {
    assert_eq!(is_bracket('a'), false);
}

#[test]
fn get_match_0() {
    assert_eq!(get_match("hi there!"), None);
}

#[test]
fn get_match_1() {
    assert_eq!(get_match("[]"), Some(('[', ']', 0, 1)));
}

#[test]
fn get_match_2() {
    assert_eq!(get_match("aa () bb"), Some(('(', ')', 3, 4)));
}

#[test]
fn get_match_3() {
    assert_eq!(get_match("[] ()"), Some(('[', ']', 0, 1)));
}

#[test]
fn get_match_4() {
    assert_eq!(get_match("[()  ] hi"), Some(('[', ']', 0, 5)));
}

#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}

#[test]
#[ignore]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}

#[test]
#[ignore]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}

#[test]
#[ignore]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}

#[test]
#[ignore]
fn wrong_closing_bracket() {
    assert!(!brackets_are_balanced("{]"));
}

#[test]
#[ignore]
fn paired_with_whitespace() {
    assert!(brackets_are_balanced("{ }"));
}

#[test]
#[ignore]
fn partially_paired_brackets() {
    assert!(!brackets_are_balanced("{[])"));
}

#[test]
#[ignore]
fn simple_nested_brackets() {
    assert!(brackets_are_balanced("{[]}"));
}

#[test]
#[ignore]
fn several_paired_brackets() {
    assert!(brackets_are_balanced("{}[]"));
}

#[test]
#[ignore]
fn paired_and_nested_brackets() {
    assert!(brackets_are_balanced("([{}({}[])])"));
}

#[test]
#[ignore]
fn unopened_closing_brackets() {
    assert!(!brackets_are_balanced("{[)][]}"));
}

#[test]
#[ignore]
fn unpaired_and_nested_brackets() {
    assert!(!brackets_are_balanced("([{])"));
}

#[test]
#[ignore]
fn paired_and_wrong_nested_brackets() {
    assert!(!brackets_are_balanced("[({]})"));
}

#[test]
#[ignore]
fn paired_and_incomplete_brackets() {
    assert!(!brackets_are_balanced("{}["));
}

#[test]
#[ignore]
fn too_many_closing_brackets() {
    assert!(!brackets_are_balanced("[]]"));
}

#[test]
#[ignore]
fn early_incomplete_brackets() {
    assert!(!brackets_are_balanced(")()"));
}

#[test]
#[ignore]
fn early_mismatched_brackets() {
    assert!(!brackets_are_balanced("{)()"));
}

#[test]
#[ignore]
fn math_expression() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

#[test]
#[ignore]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(brackets_are_balanced(input));
}
