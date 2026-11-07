use super::*;

#[test]
fn ids_are_one_segment_each_and_names_keep_their_prefix() {
    assert_eq!(
        attributes_path("1", "2", "3"),
        "/workspaces/1/custom-connections/2/resources/3/attributes"
    );
    assert_eq!(
        attributes_path("1", "name:HR System", "name:Emp/loyees"),
        "/workspaces/1/custom-connections/name:HR%20System/resources/name:Emp%2Floyees/attributes"
    );
}
